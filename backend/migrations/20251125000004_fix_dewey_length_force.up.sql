-- Force increase dewey_code column length to 255
ALTER TABLE titles MODIFY COLUMN dewey_code VARCHAR(255);
