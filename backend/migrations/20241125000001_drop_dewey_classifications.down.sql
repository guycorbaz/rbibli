-- Recreate dewey_classifications table (inverse of up.sql)
CREATE TABLE dewey_classifications (
    code VARCHAR(20) PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    level INT NOT NULL,
    parent_code VARCHAR(20),
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FULLTEXT INDEX idx_search (name, description),
    INDEX idx_parent (parent_code),
    FOREIGN KEY (parent_code) REFERENCES dewey_classifications(code) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
