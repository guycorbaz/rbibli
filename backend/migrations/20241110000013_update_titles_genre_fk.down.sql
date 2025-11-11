-- Remove foreign key constraint
ALTER TABLE titles DROP FOREIGN KEY fk_titles_genre;

-- Remove index
DROP INDEX idx_titles_genre_id ON titles;

-- Remove genre_id column
ALTER TABLE titles DROP COLUMN genre_id;

-- Restore original genre column name
ALTER TABLE titles CHANGE COLUMN genre_old genre VARCHAR(100);
