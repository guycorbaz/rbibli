-- Add series_id and series_number columns to titles table
ALTER TABLE titles
ADD COLUMN series_id CHAR(36),
ADD COLUMN series_number VARCHAR(50);

-- Add foreign key constraint
ALTER TABLE titles
ADD CONSTRAINT fk_titles_series
FOREIGN KEY (series_id) REFERENCES series(id)
ON DELETE SET NULL;

-- Create index for faster series lookups
CREATE INDEX idx_titles_series_id ON titles(series_id);
