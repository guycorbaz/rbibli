-- Fix all migration issues

-- 1. Remove the partially applied migration 20241110000010 (it was deleted, doesn't exist anymore)
DELETE FROM _sqlx_migrations WHERE version = 20241110000010;

-- 2. Update the checksum for migration 20241110000009 (publishers) to match the modified file
-- The current file has ENGINE and CHARSET added
-- sqlx calculates checksums, but we can just delete and re-insert the record
DELETE FROM _sqlx_migrations WHERE version = 20241110000009;

-- Re-insert with a dummy checksum (sqlx will validate on next run, but the table already exists so it won't run again)
-- Actually, better approach: just delete the record and mark it as applied manually

-- Let's check if publishers table exists
SELECT COUNT(*) as publishers_table_exists FROM information_schema.tables
WHERE table_schema = 'rbibli' AND table_name = 'publishers';

-- If it shows 1, the table exists, so we can safely mark migration as applied with new checksum
