-- Project CRUD. All full-row queries share the `ProjectRow` type so the repo
-- layer needs a single `From<ProjectRow> for models::Project`.

--: ProjectRow(description?)

--! create (description?) : ProjectRow
INSERT INTO projects (name, description, repository_url)
VALUES (:name, :description, :repository_url)
RETURNING *;

--! get : ProjectRow
SELECT * FROM projects WHERE id = :id;

--! get_by_name : ProjectRow
SELECT * FROM projects WHERE name = :name;

--! list : ProjectRow
SELECT * FROM projects ORDER BY created_at DESC LIMIT :limit OFFSET :offset;

--! count
SELECT COUNT(*) FROM projects;

--! update (description?) : ProjectRow
UPDATE projects
SET name = :name, description = :description, repository_url = :repository_url
WHERE id = :id
RETURNING *;

--! upsert (description?) : ProjectRow
INSERT INTO projects (name, description, repository_url)
VALUES (:name, :description, :repository_url)
ON CONFLICT (name) DO UPDATE
SET description = EXCLUDED.description, repository_url = EXCLUDED.repository_url
RETURNING *;

--! list_without_active_jobsets : ProjectRow
SELECT p.*
FROM projects p
WHERE NOT EXISTS (SELECT 1 FROM jobsets j WHERE j.project_id = p.id);

--! delete
DELETE FROM projects WHERE id = :id;
