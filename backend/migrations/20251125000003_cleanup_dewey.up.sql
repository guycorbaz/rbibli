-- Cleanup dewey columns
-- Drop dewey_category as it is unused
-- Ensure dewey_code is VARCHAR(100)

ALTER TABLE titles
DROP COLUMN dewey_category,
MODIFY COLUMN dewey_code VARCHAR(100);
