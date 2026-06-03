//! TLS-aware tokio-postgres connection helpers, shared by the migration runner
//! and `circus-common`'s deadpool pool builder.
//!
//! The connection's TLS posture is driven by the `sslmode` query parameter in
//! the database URL, mirroring libpq: `disable` -> plaintext, `verify-ca` /
//! `verify-full` -> verified against the webpki roots, anything else (including
//! absent) -> encrypted but unverified.

use std::sync::{Arc, Once};

use rustls::{
  DigitallySignedStruct,
  SignatureScheme,
  client::danger::{
    HandshakeSignatureValid,
    ServerCertVerified,
    ServerCertVerifier,
  },
  pki_types::{CertificateDer, ServerName, UnixTime},
};
use tokio_postgres::NoTls;
use tokio_postgres_rustls::MakeRustlsConnect;
use url::Url;

/// Resolved TLS behaviour for a connection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TlsMode {
  Disable,
  Unverified,
  VerifyFull,
}

/// Determine the TLS mode from a database URL's `sslmode` query parameter.
#[must_use]
pub fn tls_mode(database_url: &str) -> TlsMode {
  let Some(sslmode) = Url::parse(database_url)
    .ok()
    .and_then(|url| sslmode_from_query(url.query()))
  else {
    return TlsMode::Unverified;
  };

  match sslmode.to_ascii_lowercase().as_str() {
    "disable" => TlsMode::Disable,
    "verify-ca" | "verify-full" => TlsMode::VerifyFull,
    _ => TlsMode::Unverified,
  }
}

fn sslmode_from_query(query: Option<&str>) -> Option<String> {
  url::form_urlencoded::parse(query?.as_bytes())
    .find(|(key, _)| key == "sslmode")
    .map(|(_, value)| value.into_owned())
}

/// Build a rustls connector. `Disable` is treated as the unverified path;
/// callers route `Disable` to [`NoTls`] before reaching this.
#[must_use]
pub fn tls_connector(mode: TlsMode) -> MakeRustlsConnect {
  static TLS_PROVIDER: Once = Once::new();
  TLS_PROVIDER.call_once(|| {
    let _ = rustls::crypto::ring::default_provider().install_default();
  });

  let builder = rustls::ClientConfig::builder();
  // `Disable` never reaches here (callers route it to NoTls); treat anything
  // that is not full verification as the unverified path.
  let config = if mode == TlsMode::VerifyFull {
    let roots: rustls::RootCertStore =
      webpki_roots::TLS_SERVER_ROOTS.iter().cloned().collect();
    builder.with_root_certificates(roots).with_no_client_auth()
  } else {
    builder
      .dangerous()
      .with_custom_certificate_verifier(Arc::new(NoCertificateVerification))
      .with_no_client_auth()
  };
  MakeRustlsConnect::new(config)
}

/// Connect a single tokio-postgres client, spawning its connection task.
///
/// # Errors
///
/// Returns an error if the TCP/TLS handshake or startup fails.
pub async fn connect_once(
  database_url: &str,
) -> Result<tokio_postgres::Client, tokio_postgres::Error> {
  match tls_mode(database_url) {
    TlsMode::Disable => {
      let (client, connection) =
        tokio_postgres::connect(database_url, NoTls).await?;
      spawn_connection(connection);
      Ok(client)
    },
    mode => {
      let (client, connection) =
        tokio_postgres::connect(database_url, tls_connector(mode)).await?;
      spawn_connection(connection);
      Ok(client)
    },
  }
}

fn spawn_connection(
  connection: impl std::future::Future<
    Output = std::result::Result<(), tokio_postgres::Error>,
  > + Send
  + 'static,
) {
  tokio::spawn(async move {
    if let Err(err) = connection.await {
      tracing::error!(?err, "postgres connection task ended with error");
    }
  });
}

#[derive(Debug)]
struct NoCertificateVerification;

impl ServerCertVerifier for NoCertificateVerification {
  fn verify_server_cert(
    &self,
    _end_entity: &CertificateDer<'_>,
    _intermediates: &[CertificateDer<'_>],
    _server_name: &ServerName<'_>,
    _ocsp_response: &[u8],
    _now: UnixTime,
  ) -> std::result::Result<ServerCertVerified, rustls::Error> {
    Ok(ServerCertVerified::assertion())
  }

  fn verify_tls12_signature(
    &self,
    _message: &[u8],
    _cert: &CertificateDer<'_>,
    _dss: &DigitallySignedStruct,
  ) -> std::result::Result<HandshakeSignatureValid, rustls::Error> {
    Ok(HandshakeSignatureValid::assertion())
  }

  fn verify_tls13_signature(
    &self,
    _message: &[u8],
    _cert: &CertificateDer<'_>,
    _dss: &DigitallySignedStruct,
  ) -> std::result::Result<HandshakeSignatureValid, rustls::Error> {
    Ok(HandshakeSignatureValid::assertion())
  }

  fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
    vec![
      SignatureScheme::ECDSA_NISTP256_SHA256,
      SignatureScheme::ECDSA_NISTP384_SHA384,
      SignatureScheme::ED25519,
      SignatureScheme::RSA_PSS_SHA256,
      SignatureScheme::RSA_PSS_SHA384,
      SignatureScheme::RSA_PSS_SHA512,
      SignatureScheme::RSA_PKCS1_SHA256,
      SignatureScheme::RSA_PKCS1_SHA384,
      SignatureScheme::RSA_PKCS1_SHA512,
    ]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tls_mode_honors_postgres_sslmode() {
    assert_eq!(
      tls_mode("postgresql://localhost/circus"),
      TlsMode::Unverified
    );
    assert_eq!(
      tls_mode("postgresql://localhost/circus?sslmode=disable"),
      TlsMode::Disable
    );
    assert_eq!(
      tls_mode("postgresql://localhost/circus?sslmode=verify-ca"),
      TlsMode::VerifyFull
    );
    assert_eq!(
      tls_mode("postgresql://localhost/circus?sslmode=verify-full"),
      TlsMode::VerifyFull
    );
  }
}
