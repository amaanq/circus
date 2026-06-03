-- Notification config CRUD. All full-row queries share the
-- `NotificationConfigRow` type so the repo layer needs a single
-- `From<NotificationConfigRow> for models::NotificationConfig`. Every column in
-- notification_configs is NOT NULL, so the row type lists no nullable columns.
--: NotificationConfigRow()
--! create : NotificationConfigRow
INSERT INTO
  notification_configs (project_id, notification_type, config)
VALUES
  (:project_id,:notification_type,:config)
RETURNING
  *;

--! list_for_project : NotificationConfigRow
SELECT
  *
FROM
  notification_configs
WHERE
  project_id =:project_id
  AND enabled = true
ORDER BY
  created_at DESC;

--! delete_for_project
DELETE FROM notification_configs
WHERE
  project_id =:project_id
  AND id =:id;

--! upsert : NotificationConfigRow
INSERT INTO
  notification_configs (project_id, notification_type, config, enabled)
VALUES
  (:project_id,:notification_type,:config,:enabled)
ON CONFLICT (project_id, notification_type) DO UPDATE
SET
  config = EXCLUDED.config,
  enabled = EXCLUDED.enabled
RETURNING
  *;

--! sync_for_project_delete
DELETE FROM notification_configs
WHERE
  project_id =:project_id
  AND notification_type != ALL (:notification_types::text[]);
