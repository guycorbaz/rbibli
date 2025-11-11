-- Revert publishers table and titles.publisher_id changes
-- Remove foreign key constraint from titles
ALTER TABLE titles DROP FOREIGN KEY fk_titles_publisher;

-- Drop index
ALTER TABLE titles DROP INDEX idx_publisher_id;

-- Remove publisher_id column
ALTER TABLE titles DROP COLUMN publisher_id;

-- Rename publisher_old back to publisher
ALTER TABLE titles CHANGE COLUMN publisher_old publisher VARCHAR(200);

-- Drop publishers table
DROP TABLE IF EXISTS publishers;
