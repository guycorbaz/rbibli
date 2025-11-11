-- Mark migrations as applied without re-running them

-- First, delete any existing records for these migrations
DELETE FROM _sqlx_migrations WHERE version >= 20241110000009;

-- Check which tables exist
SELECT
    'publishers' as table_name,
    IF(COUNT(*) > 0, 'EXISTS', 'MISSING') as status
FROM information_schema.tables
WHERE table_schema = 'rbibli' AND table_name = 'publishers'
UNION ALL
SELECT
    'genres' as table_name,
    IF(COUNT(*) > 0, 'EXISTS', 'MISSING') as status
FROM information_schema.tables
WHERE table_schema = 'rbibli' AND table_name = 'genres'
UNION ALL
SELECT
    'titles.genre_id' as table_name,
    IF(COUNT(*) > 0, 'EXISTS', 'MISSING') as status
FROM information_schema.columns
WHERE table_schema = 'rbibli' AND table_name = 'titles' AND column_name = 'genre_id';

-- Create genres table if it doesn't exist
CREATE TABLE IF NOT EXISTS genres (
    id CHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_genres_name (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Update titles table if needed
-- Check if we need to rename genre column
SET @column_exists = (SELECT COUNT(*) FROM information_schema.columns
    WHERE table_schema = 'rbibli' AND table_name = 'titles' AND column_name = 'genre');

SET @genre_old_exists = (SELECT COUNT(*) FROM information_schema.columns
    WHERE table_schema = 'rbibli' AND table_name = 'titles' AND column_name = 'genre_old');

-- Rename if needed
SET @sql = IF(@column_exists > 0 AND @genre_old_exists = 0,
    'ALTER TABLE titles CHANGE COLUMN `genre` `genre_old` VARCHAR(100)',
    'SELECT "genre column already renamed or does not exist" as info');
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

-- Add FK constraint if it doesn't exist
SET @fk_exists = (SELECT COUNT(*) FROM information_schema.table_constraints
    WHERE constraint_schema = 'rbibli' AND table_name = 'titles' AND constraint_name = 'fk_titles_genre');

SET @sql = IF(@fk_exists = 0,
    'ALTER TABLE titles ADD CONSTRAINT fk_titles_genre FOREIGN KEY (genre_id) REFERENCES genres(id) ON DELETE SET NULL',
    'SELECT "FK constraint already exists" as info');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Add index if it doesn't exist
SET @idx_exists = (SELECT COUNT(*) FROM information_schema.statistics
    WHERE table_schema = 'rbibli' AND table_name = 'titles' AND index_name = 'idx_titles_genre_id');

SET @sql = IF(@idx_exists = 0,
    'CREATE INDEX idx_titles_genre_id ON titles(genre_id)',
    'SELECT "Index already exists" as info');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SELECT 'Schema changes applied successfully!' as result;
SELECT 'Now manually mark migrations as applied using the command below' as next_step;
