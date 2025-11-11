-- Apply genres migrations manually

-- 1. Remove problematic migration records
DELETE FROM _sqlx_migrations WHERE version IN (20241110000009, 20241110000010);

-- 2. Check if publishers table exists (should exist)
SELECT 'Checking publishers table...' as status;
SELECT COUNT(*) as publishers_exists FROM information_schema.tables
WHERE table_schema = 'rbibli' AND table_name = 'publishers';

-- 3. Create genres table
SELECT 'Creating genres table...' as status;
CREATE TABLE IF NOT EXISTS genres (
    id CHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_genres_name (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 4. Update titles table to add genre_id FK
SELECT 'Updating titles table...' as status;

-- Check if genre_old already exists
SELECT COUNT(*) as genre_old_exists FROM information_schema.columns
WHERE table_schema = 'rbibli' AND table_name = 'titles' AND column_name = 'genre_old';

-- Rename genre to genre_old if not already done
SET @rename_query = IF(
    (SELECT COUNT(*) FROM information_schema.columns WHERE table_schema = 'rbibli' AND table_name = 'titles' AND column_name = 'genre') > 0,
    'ALTER TABLE titles CHANGE COLUMN genre genre_old VARCHAR(100)',
    'SELECT "genre already renamed" as status'
);
PREPARE stmt FROM @rename_query;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Add genre_id column if not exists
SET @add_column_query = IF(
    (SELECT COUNT(*) FROM information_schema.columns WHERE table_schema = 'rbibli' AND table_name = 'titles' AND column_name = 'genre_id') = 0,
    'ALTER TABLE titles ADD COLUMN genre_id CHAR(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci',
    'SELECT "genre_id already exists" as status'
);
PREPARE stmt FROM @add_column_query;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Add foreign key constraint if not exists
SET @fk_exists = (SELECT COUNT(*) FROM information_schema.table_constraints
    WHERE constraint_schema = 'rbibli' AND table_name = 'titles' AND constraint_name = 'fk_titles_genre');

SET @add_fk_query = IF(@fk_exists = 0,
    'ALTER TABLE titles ADD CONSTRAINT fk_titles_genre FOREIGN KEY (genre_id) REFERENCES genres(id) ON DELETE SET NULL',
    'SELECT "FK already exists" as status'
);
PREPARE stmt FROM @add_fk_query;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Add index if not exists
SET @idx_exists = (SELECT COUNT(*) FROM information_schema.statistics
    WHERE table_schema = 'rbibli' AND table_name = 'titles' AND index_name = 'idx_titles_genre_id');

SET @add_idx_query = IF(@idx_exists = 0,
    'CREATE INDEX idx_titles_genre_id ON titles(genre_id)',
    'SELECT "Index already exists" as status'
);
PREPARE stmt FROM @add_idx_query;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SELECT 'Migrations applied successfully!' as status;
SELECT 'Now you need to re-run: sqlx migrate run' as next_step;
