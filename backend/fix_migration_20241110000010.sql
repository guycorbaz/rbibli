-- Fix partially applied migration 20241110000010
-- Remove the partially applied migration record
DELETE FROM _sqlx_migrations WHERE version = 20241110000010;

-- Now we can reapply migrations
