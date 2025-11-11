-- Create publishers table
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
);
