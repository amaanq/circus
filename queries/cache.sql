-- Binary-cache lookup helpers used by narinfo serving.

--! find_signed_product_path
SELECT
  bp.path AS path
FROM
  build_products bp
  JOIN builds b ON b.id = bp.build_id
WHERE
  bp.path LIKE :like_pattern
  AND b.signed = true
LIMIT
  1;

--! find_signed_build_output_path : (path)
SELECT
  build_output_path AS path
FROM
  builds
WHERE
  build_output_path LIKE :like_pattern
  AND signed = true
LIMIT
  1;
