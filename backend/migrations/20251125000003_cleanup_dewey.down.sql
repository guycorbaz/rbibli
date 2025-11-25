-- Revert cleanup dewey columns

ALTER TABLE titles
ADD COLUMN dewey_category VARCHAR(200),
MODIFY COLUMN dewey_code VARCHAR(100);
