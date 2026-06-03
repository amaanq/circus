-- Evaluation CRUD. All full-row queries share the `EvaluationRow` type so the
-- repo layer needs a single `From<EvaluationRow> for models::Evaluation`.

--: EvaluationRow(error_message?, inputs_hash?, pr_number?, pr_head_branch?, pr_base_branch?, pr_action?)

--! create_with_kind (pr_number?, pr_head_branch?, pr_base_branch?, pr_action?) : EvaluationRow
INSERT INTO evaluations (
  jobset_id, commit_hash, status, trigger_kind,
  pr_number, pr_head_branch, pr_base_branch, pr_action
)
VALUES (
  :jobset_id, :commit_hash, :status, :trigger_kind,
  :pr_number, :pr_head_branch, :pr_base_branch, :pr_action
)
RETURNING *;

--! get : EvaluationRow
SELECT * FROM evaluations WHERE id = :id;

--! get_visible : EvaluationRow
SELECT * FROM evaluations
WHERE id = :id AND (:include_hidden::boolean OR hidden = false);

--! list_for_jobset : EvaluationRow
SELECT * FROM evaluations WHERE jobset_id = :jobset_id ORDER BY evaluation_time DESC;

--! list_filtered_with_visibility (jobset_id?, status?) : EvaluationRow
SELECT * FROM evaluations
WHERE (:jobset_id::uuid IS NULL OR jobset_id = :jobset_id)
  AND (:status::text IS NULL OR status = :status)
  AND (:include_hidden::boolean OR hidden = false)
ORDER BY evaluation_time DESC
LIMIT :limit OFFSET :offset;

--! count_filtered_with_visibility (jobset_id?, status?)
SELECT COUNT(*) FROM evaluations
WHERE (:jobset_id::uuid IS NULL OR jobset_id = :jobset_id)
  AND (:status::text IS NULL OR status = :status)
  AND (:include_hidden::boolean OR hidden = false);

--! set_hidden : EvaluationRow
UPDATE evaluations SET hidden = :hidden WHERE id = :id RETURNING *;

--! try_claim_pending : EvaluationRow
UPDATE evaluations SET status = 'running'
WHERE id = :id AND status = 'pending'
RETURNING *;

--! update_status (error_message?) : EvaluationRow
UPDATE evaluations SET status = :status, error_message = :error_message
WHERE id = :id
RETURNING *;

--! get_latest : EvaluationRow
SELECT * FROM evaluations
WHERE jobset_id = :jobset_id AND status = 'completed'
ORDER BY evaluation_time DESC
LIMIT 1;

--! set_inputs_hash
UPDATE evaluations SET inputs_hash = :inputs_hash WHERE id = :id;

--! get_by_inputs_hash : EvaluationRow
SELECT * FROM evaluations
WHERE jobset_id = :jobset_id AND inputs_hash = :inputs_hash AND status = 'completed'
ORDER BY evaluation_time DESC
LIMIT 1;

--! count
SELECT COUNT(*) FROM evaluations;

--! list_pending : EvaluationRow
SELECT * FROM evaluations WHERE status = 'pending' ORDER BY evaluation_time ASC;

--! list_jobsets_with_pending
SELECT DISTINCT jobset_id FROM evaluations WHERE status = 'pending';

--! get_by_jobset_and_commit : EvaluationRow
SELECT * FROM evaluations
WHERE jobset_id = :jobset_id AND commit_hash = :commit_hash
ORDER BY (trigger_kind = 'interval') ASC, evaluation_time DESC
LIMIT 1;
