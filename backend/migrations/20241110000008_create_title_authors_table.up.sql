-- Create title_authors junction table for many-to-many relationship
CREATE TABLE title_authors (
    id CHAR(36) PRIMARY KEY,
    title_id CHAR(36) NOT NULL,
    author_id CHAR(36) NOT NULL,
    role ENUM('main_author', 'co_author', 'translator', 'illustrator', 'editor') NOT NULL DEFAULT 'main_author',
    display_order INT NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (title_id) REFERENCES titles(id) ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES authors(id) ON DELETE CASCADE,
    UNIQUE KEY unique_title_author_role (title_id, author_id, role),
    INDEX idx_title_id (title_id),
    INDEX idx_author_id (author_id),
    INDEX idx_role (role)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
