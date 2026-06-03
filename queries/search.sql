-- Stable search helpers. The advanced search queries remain dynamic in Rust
-- because their WHERE and ORDER BY clauses are assembled from optional filters.

--: ProjectQuickSearchRow(description?)
--: BuildQuickSearchRow(started_at?, completed_at?, log_path?, build_output_path?, error_message?, system?, notification_pending_since?, outputs?, constituents?, builder_id?, agent_machine_id?, fod_hash?, meta_description?, meta_license?, meta_homepage?, meta_maintainers?, started_notified_at?)

--! quick_projects : ProjectQuickSearchRow
SELECT
  *
FROM
  projects
WHERE
  name ILIKE :pattern
  OR description ILIKE :pattern
ORDER BY
  name
LIMIT
  :limit;

--! quick_builds : BuildQuickSearchRow
SELECT
  *
FROM
  builds
WHERE
  job_name ILIKE :pattern
  OR drv_path ILIKE :pattern
ORDER BY
  created_at DESC
LIMIT
  :limit;
