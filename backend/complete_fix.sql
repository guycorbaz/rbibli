-- Complete fix for migration issues
-- Run this script, then run: sqlx migrate run

-- Step 1: Remove all problematic migration records
DELETE FROM _sqlx_migrations WHERE version >= 20241110000009;

-- Step 2: Ensure publishers table exists with correct structure
-- (should already exist, this is just to be safe)
CREATE TABLE IF NOT EXISTS publishers (
    id CHAR(36) PRIMARY KEY,
    name VARCHAR(300) NOT NULL,
    description TEXT,
    website_url VARCHAR(500),
    country VARCHAR(100),
    founded_year INT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_publishers_name (name),
    INDEX idx_publishers_country (country)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Step 3: Ensure titles.publisher_id exists
-- Add publisher_id column if it doesn't exist
SET @pub_id_exists = (SELECT COUNT(*) FROM information_schema.columns
    WHERE table_schema = 'rbibli' AND table_name = 'titles' AND column_name = 'publisher_id');

SET @sql = IF(@pub_id_exists = 0,
    'ALTER TABLE titles ADD COLUMN publisher_id CHAR(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci',
    'SELECT "publisher_id already exists" as info');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Add FK for publisher_id if it doesn't exist
SET @pub_fk_exists = (SELECT COUNT(*) FROM information_schema.table_constraints
    WHERE constraint_schema = 'rbibli' AND table_name = 'titles' AND constraint_name = 'fk_titles_publisher');

SET @sql = IF(@pub_fk_exists = 0,
    'ALTER TABLE titles ADD CONSTRAINT fk_titles_publisher FOREIGN KEY (publisher_id) REFERENCES publishers(id) ON DELETE SET NULL',
    'SELECT "publisher FK already exists" as info');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Step 4: Create genres table
CREATE TABLE IF NOT EXISTS genres (
    id CHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_genres_name (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Step 5: Update titles table for genres
-- Check if genre column exists and rename to genre_old
SET @genre_exists = (SELECT COUNT(*) FROM information_schema.columns
    WHERE table_schema = 'rbibli' AND table_name = 'titles' AND column_name = 'genre');

SET @genre_old_exists = (SELECT COUNT(*) FROM information_schema.columns
    WHERE table_schema = 'rbibli' AND table_name = 'titles' AND column_name = 'genre_old');

-- Only rename if genre exists and genre_old doesn't
SET @sql = IF(@genre_exists > 0 AND @genre_old_exists = 0,
    'ALTER TABLE titles CHANGE COLUMN `genre` `genre_old` VARCHAR(100)',
    'SELECT "genre already processed" as info');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Add genre_id if it doesn't exist
SET @genre_id_exists = (SELECT COUNT(*) FROM information_schema.columns
    WHERE table_schema = 'rbibli' AND table_name = 'titles' AND column_name = 'genre_id');

SET @sql = IF(@genre_id_exists = 0,
    'ALTER TABLE titles ADD COLUMN genre_id CHAR(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci',
    'SELECT "genre_id already exists" as info');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Add FK for genre if it doesn't exist
SET @genre_fk_exists = (SELECT COUNT(*) FROM information_schema.table_constraints
    WHERE constraint_schema = 'rbibli' AND table_name = 'titles' AND constraint_name = 'fk_titles_genre');

SET @sql = IF(@genre_fk_exists = 0,
    'ALTER TABLE titles ADD CONSTRAINT fk_titles_genre FOREIGN KEY (genre_id) REFERENCES genres(id) ON DELETE SET NULL',
    'SELECT "genre FK already exists" as info');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Add index for genre_id if it doesn't exist
SET @genre_idx_exists = (SELECT COUNT(*) FROM information_schema.statistics
    WHERE table_schema = 'rbibli' AND table_name = 'titles' AND index_name = 'idx_titles_genre_id');

SET @sql = IF(@genre_idx_exists = 0,
    'CREATE INDEX idx_titles_genre_id ON titles(genre_id)',
    'SELECT "genre index already exists" as info');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SELECT 'All schema changes have been applied!' as status;
SELECT 'Now run: sqlx migrate run' as next_step;
SELECT 'It should mark migrations 000009, 000011, 000012, 000013 as applied' as info;
