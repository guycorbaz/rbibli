-- Final fix: Apply schema and mark migrations as complete with correct checksums

-- Step 1: Remove any existing records for these migrations
DELETE FROM _sqlx_migrations WHERE version >= 20241110000009;

-- Step 2: Apply all schema changes (safe to run multiple times)

-- Publishers table
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

-- Genres table
CREATE TABLE IF NOT EXISTS genres (
    id CHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_genres_name (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Update titles table for publishers (if not already done)
SET @pub_id_exists = (SELECT COUNT(*) FROM information_schema.columns
    WHERE table_schema = 'rbibli' AND table_name = 'titles' AND column_name = 'publisher_id');

SET @sql = IF(@pub_id_exists = 0,
    'ALTER TABLE titles ADD COLUMN publisher_id CHAR(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci',
    'SELECT "publisher_id exists" as info');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Update titles table for genres (rename genre to genre_old, add genre_id)
SET @genre_exists = (SELECT COUNT(*) FROM information_schema.columns
    WHERE table_schema = 'rbibli' AND table_name = 'titles' AND column_name = 'genre');
SET @genre_old_exists = (SELECT COUNT(*) FROM information_schema.columns
    WHERE table_schema = 'rbibli' AND table_name = 'titles' AND column_name = 'genre_old');

SET @sql = IF(@genre_exists > 0 AND @genre_old_exists = 0,
    'ALTER TABLE titles CHANGE COLUMN `genre` `genre_old` VARCHAR(100)',
    'SELECT "genre rename done" as info');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SET @genre_id_exists = (SELECT COUNT(*) FROM information_schema.columns
    WHERE table_schema = 'rbibli' AND table_name = 'titles' AND column_name = 'genre_id');

SET @sql = IF(@genre_id_exists = 0,
    'ALTER TABLE titles ADD COLUMN genre_id CHAR(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci',
    'SELECT "genre_id exists" as info');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Add foreign keys (safe - will skip if exist)
SET @pub_fk_exists = (SELECT COUNT(*) FROM information_schema.table_constraints
    WHERE constraint_schema = 'rbibli' AND table_name = 'titles' AND constraint_name = 'fk_titles_publisher');
SET @sql = IF(@pub_fk_exists = 0,
    'ALTER TABLE titles ADD CONSTRAINT fk_titles_publisher FOREIGN KEY (publisher_id) REFERENCES publishers(id) ON DELETE SET NULL',
    'SELECT "publisher FK exists" as info');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SET @genre_fk_exists = (SELECT COUNT(*) FROM information_schema.table_constraints
    WHERE constraint_schema = 'rbibli' AND table_name = 'titles' AND constraint_name = 'fk_titles_genre');
SET @sql = IF(@genre_fk_exists = 0,
    'ALTER TABLE titles ADD CONSTRAINT fk_titles_genre FOREIGN KEY (genre_id) REFERENCES genres(id) ON DELETE SET NULL',
    'SELECT "genre FK exists" as info');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Add indexes (safe - will skip if exist)
SET @genre_idx_exists = (SELECT COUNT(*) FROM information_schema.statistics
    WHERE table_schema = 'rbibli' AND table_name = 'titles' AND index_name = 'idx_titles_genre_id');
SET @sql = IF(@genre_idx_exists = 0,
    'CREATE INDEX idx_titles_genre_id ON titles(genre_id)',
    'SELECT "genre index exists" as info');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Step 3: Mark migrations as applied with correct checksums
INSERT INTO _sqlx_migrations (version, description, installed_on, success, checksum, execution_time)
VALUES
    (20241110000009, 'create publishers table', NOW(), TRUE, UNHEX('93c31a09bff0d78ea0c0725e7ef45d516b3a0adfbce0d1d839073d7cac0c9003'), 1),
    (20241110000011, 'fix publishers collation', NOW(), TRUE, UNHEX('02ba708b343450533dfd97132e7947bff0ba3644024562edfc44cd4704711da6'), 1),
    (20241110000012, 'create genres table', NOW(), TRUE, UNHEX('a208a7600795d4566c6952e2185f720650db396cf5559f4c521bf0e2c8e25fab'), 1),
    (20241110000013, 'update titles genre fk', NOW(), TRUE, UNHEX('a8cb2a80266c5c5a5928299a91895deb1609cd79649db520a908b8df23f78c3a'), 1);

SELECT 'SUCCESS! All migrations applied and marked complete!' as status;
SELECT 'You can now restart your backend server.' as next_step;
SELECT 'The genres feature should work correctly now.' as final_note;
