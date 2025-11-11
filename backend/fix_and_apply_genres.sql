-- Fix migration issues and apply genres migrations

-- Step 1: Remove problematic migration records from tracking table
DELETE FROM _sqlx_migrations WHERE version IN (20241110000009, 20241110000010);

-- Step 2: Create genres table
CREATE TABLE IF NOT EXISTS genres (
    id CHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_genres_name (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Step 3: Update titles table for genres FK
-- Rename genre column to genre_old (if column 'genre' exists)
ALTER TABLE titles CHANGE COLUMN `genre` `genre_old` VARCHAR(100);

-- Add genre_id column
ALTER TABLE titles ADD COLUMN genre_id CHAR(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- Add foreign key constraint
ALTER TABLE titles ADD CONSTRAINT fk_titles_genre
    FOREIGN KEY (genre_id) REFERENCES genres(id) ON DELETE SET NULL;

-- Add index
CREATE INDEX idx_titles_genre_id ON titles(genre_id);

-- Done! Now you can run: sqlx migrate run
