-- Check current collations
SELECT
    TABLE_NAME,
    COLUMN_NAME,
    CHARACTER_SET_NAME,
    COLLATION_NAME
FROM INFORMATION_SCHEMA.COLUMNS
WHERE TABLE_SCHEMA = 'rbibli'
AND TABLE_NAME IN ('publishers', 'titles')
AND COLUMN_NAME IN ('id', 'publisher_id')
ORDER BY TABLE_NAME, COLUMN_NAME;

-- Drop foreign key constraint first
ALTER TABLE titles DROP FOREIGN KEY IF EXISTS fk_titles_publisher;

-- Modify publisher_id column to match publishers.id collation exactly
ALTER TABLE titles MODIFY COLUMN publisher_id CHAR(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- Modify publishers.id to ensure it has the correct collation
ALTER TABLE publishers MODIFY COLUMN id CHAR(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- Recreate the foreign key constraint
ALTER TABLE titles ADD CONSTRAINT fk_titles_publisher
    FOREIGN KEY (publisher_id) REFERENCES publishers(id) ON DELETE SET NULL;

-- Verify the changes
SELECT
    TABLE_NAME,
    COLUMN_NAME,
    CHARACTER_SET_NAME,
    COLLATION_NAME
FROM INFORMATION_SCHEMA.COLUMNS
WHERE TABLE_SCHEMA = 'rbibli'
AND TABLE_NAME IN ('publishers', 'titles')
AND COLUMN_NAME IN ('id', 'publisher_id')
ORDER BY TABLE_NAME, COLUMN_NAME;
