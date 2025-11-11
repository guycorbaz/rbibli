-- Create titles table
CREATE TABLE titles (
    id CHAR(36) PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    subtitle VARCHAR(500),
    isbn VARCHAR(20),
    publisher VARCHAR(200),
    publication_year INT,
    pages INT,
    language VARCHAR(10) NOT NULL DEFAULT 'fr',
    dewey_code VARCHAR(20),
    dewey_category VARCHAR(200),
    genre VARCHAR(100),
    summary TEXT,
    cover_url VARCHAR(500),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_title (title),
    INDEX idx_isbn (isbn),
    INDEX idx_dewey_code (dewey_code),
    INDEX idx_genre (genre)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
