-- Add image BLOB storage columns to titles table
-- This migration adds support for storing cover images directly in the database

-- Add column for storing image binary data (MEDIUMBLOB supports up to 16MB)
ALTER TABLE titles ADD COLUMN image_data MEDIUMBLOB;

-- Add column for storing the MIME type of the image (e.g., 'image/jpeg', 'image/png')
ALTER TABLE titles ADD COLUMN image_mime_type VARCHAR(50);

-- Add column for storing the original filename (optional, for reference)
ALTER TABLE titles ADD COLUMN image_filename VARCHAR(255);
