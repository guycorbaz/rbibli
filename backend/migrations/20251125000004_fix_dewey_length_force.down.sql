-- Revert dewey_code column length to 100
ALTER TABLE titles MODIFY COLUMN dewey_code VARCHAR(100);
