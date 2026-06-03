-- Read/list of persistent builder agent sessions. All full-row queries share
-- the `BuilderSessionRow` type so the repo layer needs a single
-- `From<BuilderSessionRow> for models::BuilderSession`.
--: BuilderSessionRow(last_seen?, load1?, load5?, load15?, mem_total?, mem_used?, store_free?, build_dir_free?, cpu_psi_avg10?, mem_psi_avg10?, io_psi_avg10?, disabled_until?, auth_token_hash?)
--! list : BuilderSessionRow
SELECT
  *
FROM
  builder_sessions
ORDER BY
  connected DESC,
  updated_at DESC;

--! list_connected : BuilderSessionRow
SELECT
  *
FROM
  builder_sessions
WHERE
  connected = TRUE
ORDER BY
  updated_at DESC;

--! get : BuilderSessionRow
SELECT
  *
FROM
  builder_sessions
WHERE
  machine_id =:machine_id;

--! register
INSERT INTO
  builder_sessions (
    machine_id,
    name,
    hostname,
    systems,
    supported_features,
    mandatory_features,
    speed_factor,
    cpu_count,
    max_jobs,
    proto_version,
    connected,
    last_seen,
    updated_at
  )
VALUES
  (
    :machine_id,
    :name,
    :hostname,
    :systems,
    :supported_features,
    :mandatory_features,
    :speed_factor,
    :cpu_count,
    :max_jobs,
    :proto_version,
    TRUE,
    NOW(),
    NOW()
  )
ON CONFLICT (machine_id) DO UPDATE
SET
  name = EXCLUDED.name,
  hostname = EXCLUDED.hostname,
  systems = EXCLUDED.systems,
  supported_features = EXCLUDED.supported_features,
  mandatory_features = EXCLUDED.mandatory_features,
  speed_factor = EXCLUDED.speed_factor,
  cpu_count = EXCLUDED.cpu_count,
  max_jobs = EXCLUDED.max_jobs,
  proto_version = EXCLUDED.proto_version,
  connected = TRUE,
  last_seen = NOW(),
  updated_at = NOW();

--! mark_disconnected
UPDATE builder_sessions
SET
  connected = FALSE,
  updated_at = NOW()
WHERE
  machine_id =:machine_id;

--! heartbeat
UPDATE builder_sessions
SET
  last_seen = NOW(),
  load1 =:load1,
  load5 =:load5,
  load15 =:load15,
  cpu_psi_avg10 =:cpu_psi_avg10,
  mem_psi_avg10 =:mem_psi_avg10,
  io_psi_avg10 =:io_psi_avg10,
  current_jobs =:current_jobs,
  mem_total =:mem_total,
  mem_used =:mem_used,
  store_free =:store_free,
  build_dir_free =:build_dir_free,
  updated_at = NOW()
WHERE
  machine_id =:machine_id;

--! record_outcome_succeeded
UPDATE builder_sessions
SET
  builds_succeeded = builds_succeeded + 1,
  consecutive_failures = 0,
  disabled_until = NULL,
  updated_at = NOW()
WHERE
  machine_id =:machine_id;

--! record_outcome_failed
UPDATE builder_sessions
SET
  builds_failed = builds_failed + 1,
  consecutive_failures = LEAST(consecutive_failures + 1, 4),
  disabled_until = NOW() + make_interval(
    secs => 60.0 * power(3, LEAST(consecutive_failures + 1, 4) - 1) + (random() * 30)::int
  ),
  updated_at = NOW()
WHERE
  machine_id =:machine_id;

--! touch
UPDATE builder_sessions
SET
  updated_at = NOW()
WHERE
  machine_id =:machine_id;

--! is_schedulable
SELECT
  disabled_until IS NULL
  OR disabled_until <= NOW()
FROM
  builder_sessions
WHERE
  machine_id =:machine_id;

--! reset_all_connected
UPDATE builder_sessions
SET
  connected = FALSE
WHERE
  connected = TRUE;
