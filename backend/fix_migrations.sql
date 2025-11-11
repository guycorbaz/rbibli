-- Clean up partially applied migration
DELETE FROM _sqlx_migrations WHERE version = 20241110000010;

-- Verify the migration state
SELECT * FROM _sqlx_migrations ORDER BY version DESC LIMIT 5;
