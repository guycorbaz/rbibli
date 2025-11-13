-- Remove index
DROP INDEX idx_titles_publisher_id ON titles;

-- Remove foreign key constraint
ALTER TABLE titles DROP FOREIGN KEY fk_titles_publisher;

-- Remove publisher_id column
ALTER TABLE titles DROP COLUMN publisher_id;

-- Rename publisher_old back to publisher
ALTER TABLE titles CHANGE COLUMN publisher_old publisher VARCHAR(200);
