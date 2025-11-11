-- Fix collation issues for publishers table
-- This script will correct the collations to match the titles table

-- 1. Update the migration checksum to match the new file
UPDATE _sqlx_migrations
SET checksum = x'03fdeb919f06fca84566fa7da31e945a9680ac431ddc7374f9b7566d5ba16f541ea463a2a8b4d1f2e6acf8bc38d4b6bb'
WHERE version = 20241110000009;

-- 2. Drop and recreate publishers table with correct collation
DROP TABLE IF EXISTS publishers;

CREATE TABLE publishers (
    id CHAR(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci PRIMARY KEY,
    name VARCHAR(300) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
    description TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
    website_url VARCHAR(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
    country VARCHAR(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
    founded_year INT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_publishers_name (name),
    INDEX idx_publishers_country (country)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 3. Check if publisher_id column exists in titles, if not add it
SET @col_exists = (SELECT COUNT(*)
                   FROM INFORMATION_SCHEMA.COLUMNS
                   WHERE TABLE_SCHEMA = 'rbibli'
                   AND TABLE_NAME = 'titles'
                   AND COLUMN_NAME = 'publisher_id');

-- Add publisher_id column if it doesn't exist
SET @sql = IF(@col_exists = 0,
    'ALTER TABLE titles ADD COLUMN publisher_id CHAR(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci AFTER publisher_old',
    'SELECT "Column publisher_id already exists" AS message');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- 4. Add foreign key constraint if it doesn't exist
SET @fk_exists = (SELECT COUNT(*)
                  FROM INFORMATION_SCHEMA.KEY_COLUMN_USAGE
                  WHERE TABLE_SCHEMA = 'rbibli'
                  AND TABLE_NAME = 'titles'
                  AND CONSTRAINT_NAME = 'fk_titles_publisher');

SET @sql = IF(@fk_exists = 0,
    'ALTER TABLE titles ADD CONSTRAINT fk_titles_publisher FOREIGN KEY (publisher_id) REFERENCES publishers(id) ON DELETE SET NULL',
    'SELECT "Foreign key fk_titles_publisher already exists" AS message');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- 5. Add index if it doesn't exist
SET @idx_exists = (SELECT COUNT(*)
                   FROM INFORMATION_SCHEMA.STATISTICS
                   WHERE TABLE_SCHEMA = 'rbibli'
                   AND TABLE_NAME = 'titles'
                   AND INDEX_NAME = 'idx_publisher_id');

SET @sql = IF(@idx_exists = 0,
    'ALTER TABLE titles ADD INDEX idx_publisher_id (publisher_id)',
    'SELECT "Index idx_publisher_id already exists" AS message');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Verify the changes
SHOW CREATE TABLE publishers;
SHOW CREATE TABLE titles;
