-- Manually mark migrations as applied after schema changes are in place
-- This prevents sqlx from trying to re-run them

-- Insert migration records for already-applied migrations
-- The checksum is computed by sqlx but we'll use dummy values
-- since the schema is already correct

INSERT INTO _sqlx_migrations (version, description, installed_on, success, checksum, execution_time)
VALUES
    (20241110000009, 'create publishers table', NOW(), TRUE, UNHEX(SHA2('publishers_applied', 256)), 0),
    (20241110000011, 'fix publishers collation', NOW(), TRUE, UNHEX(SHA2('collation_applied', 256)), 0),
    (20241110000012, 'create genres table', NOW(), TRUE, UNHEX(SHA2('genres_applied', 256)), 0),
    (20241110000013, 'update titles genre fk', NOW(), TRUE, UNHEX(SHA2('genre_fk_applied', 256)), 0)
ON DUPLICATE KEY UPDATE success = TRUE;

SELECT 'Migrations marked as applied!' as status;
SELECT 'You can now restart your backend server' as next_step;
