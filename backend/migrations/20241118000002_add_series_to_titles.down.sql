-- Remove series columns from titles table
ALTER TABLE titles
DROP FOREIGN KEY fk_titles_series;

ALTER TABLE titles
DROP INDEX idx_titles_series_id;

ALTER TABLE titles
DROP COLUMN series_id,
DROP COLUMN series_number;
