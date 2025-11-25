-- Revert dewey_code column length to 20
ALTER TABLE titles MODIFY COLUMN dewey_code VARCHAR(20);
