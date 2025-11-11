-- Rename existing genre column to genre_old (preserve data)
ALTER TABLE titles CHANGE COLUMN genre genre_old VARCHAR(100);

-- Add genre_id column as foreign key
ALTER TABLE titles ADD COLUMN genre_id CHAR(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- Add foreign key constraint
ALTER TABLE titles
ADD CONSTRAINT fk_titles_genre
FOREIGN KEY (genre_id) REFERENCES genres(id)
ON DELETE SET NULL;

-- Add index for better query performance
CREATE INDEX idx_titles_genre_id ON titles(genre_id);
