-- Fix publishers table collation to match titles table
-- Drop and recreate publishers table with correct collation
DROP TABLE IF EXISTS publishers;

CREATE TABLE publishers (
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

-- Now update titles table to add publisher_id foreign key
-- First rename the old column if it exists
ALTER TABLE titles CHANGE COLUMN publisher publisher_old VARCHAR(200);

-- Add new publisher_id column as foreign key
ALTER TABLE titles ADD COLUMN publisher_id CHAR(36) AFTER publisher_old;

-- Add foreign key constraint
ALTER TABLE titles ADD CONSTRAINT fk_titles_publisher
    FOREIGN KEY (publisher_id) REFERENCES publishers(id) ON DELETE SET NULL;

-- Add index for better performance
ALTER TABLE titles ADD INDEX idx_publisher_id (publisher_id);
