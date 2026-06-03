#!/usr/bin/env bash
# Regenerate the committed clorinde crate at db/circus-codegen.
#
# Spins up a throwaway PostgreSQL cluster, applies every migration in
# migrations/ in order, then runs `clorinde live` over queries/*.sql so the
# generated code is type-checked against the real schema.
#
# Requires `initdb`/`pg_ctl`/`psql` (postgresql), `clorinde`, and `rustfmt`
# on PATH. From a checkout that is not in the dev shell:
#
#   nix shell nixpkgs#postgresql_18 nixpkgs#clorinde nixpkgs#rustfmt \
#     -c bash scripts/codegen.sh
#
# The same script backs the `codegen-up-to-date` flake check.
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

workdir="$(mktemp -d)"
pgdata="$workdir/pgdata"
sock="$workdir/sock"
mkdir -p "$sock"

cleanup() {
  pg_ctl -D "$pgdata" -m immediate stop >/dev/null 2>&1 || true
  rm -rf "$workdir"
}
trap cleanup EXIT

echo "==> initdb"
initdb -D "$pgdata" -U postgres --auth=trust --no-sync >/dev/null

echo "==> starting postgres on unix socket $sock"
pg_ctl -D "$pgdata" \
  -o "-k $sock -c listen_addresses='' -c fsync=off" \
  -w start >/dev/null

export PGHOST="$sock"
export PGUSER=postgres

createdb -h "$sock" -U postgres circus_codegen

echo "==> applying migrations"
for f in crates/migrations/migrations/[0-9]*.sql; do
  echo "    $f"
  psql -v ON_ERROR_STOP=1 -h "$sock" -U postgres -d circus_codegen -q -f "$f"
done

echo "==> clorinde live"
clorinde live "host=$sock user=postgres dbname=circus_codegen"

echo "==> generated db/circus-codegen"
