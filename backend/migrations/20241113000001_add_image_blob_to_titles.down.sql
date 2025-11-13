-- Revert image BLOB storage columns from titles table

ALTER TABLE titles DROP COLUMN image_filename;
ALTER TABLE titles DROP COLUMN image_mime_type;
ALTER TABLE titles DROP COLUMN image_data;
