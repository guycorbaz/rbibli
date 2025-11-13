-- Rename existing publisher column to publisher_old (preserve data)
ALTER TABLE titles CHANGE COLUMN publisher publisher_old VARCHAR(200);

-- Add publisher_id column as foreign key
ALTER TABLE titles ADD COLUMN publisher_id CHAR(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- Add foreign key constraint
ALTER TABLE titles
ADD CONSTRAINT fk_titles_publisher
FOREIGN KEY (publisher_id) REFERENCES publishers(id)
ON DELETE SET NULL;

-- Add index for better query performance
CREATE INDEX idx_titles_publisher_id ON titles(publisher_id);
