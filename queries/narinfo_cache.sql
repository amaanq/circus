-- Narinfo cache read/write. All full-row queries share the `NarinfoCacheRow`
-- type so the repo layer needs a single `From<NarinfoCacheRow> for NarInfo`.
--: NarinfoCacheRow(file_hash?, file_size?, deriver?, sig?, ca?)
--! upsert (file_hash?, file_size?, deriver?, sig?, ca?)
INSERT INTO
  narinfo_cache (
    store_path,
    nar_hash,
    nar_size,
    file_hash,
    file_size,
    compression,
    url,
    deriver,
    "references",
    sig,
    ca,
    updated_at
  )
VALUES
  (
:store_path,
:nar_hash,
:nar_size,
:file_hash,
:file_size,
:compression,
:url,
:deriver,
:references,
:sig,
:ca,
    NOW()
  )
ON CONFLICT (store_path) DO UPDATE
SET
  nar_hash = EXCLUDED.nar_hash,
  nar_size = EXCLUDED.nar_size,
  file_hash = EXCLUDED.file_hash,
  file_size = EXCLUDED.file_size,
  compression = EXCLUDED.compression,
  url = EXCLUDED.url,
  deriver = EXCLUDED.deriver,
  "references" = EXCLUDED."references",
  sig = EXCLUDED.sig,
  ca = EXCLUDED.ca,
  updated_at = NOW();

--! get : NarinfoCacheRow
SELECT
  *
FROM
  narinfo_cache
WHERE
  store_path =:store_path;

--! get_by_hash_part : NarinfoCacheRow
SELECT
  *
FROM
  narinfo_cache
WHERE
  store_path LIKE :hash_part_pattern;

--! get_by_url : NarinfoCacheRow
SELECT
  *
FROM
  narinfo_cache
WHERE
  url =:url
ORDER BY
  updated_at DESC
LIMIT
  1;

--! count
SELECT
  COUNT(*)
FROM
  narinfo_cache;
