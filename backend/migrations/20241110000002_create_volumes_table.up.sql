-- Create volumes table
CREATE TABLE volumes (
    id CHAR(36) PRIMARY KEY,
    title_id CHAR(36) NOT NULL,
    copy_number INT NOT NULL,
    barcode VARCHAR(50) NOT NULL UNIQUE,
    `condition` ENUM('excellent', 'good', 'fair', 'poor', 'damaged') NOT NULL DEFAULT 'good',
    location VARCHAR(200),
    loan_status ENUM('available', 'loaned', 'overdue', 'lost', 'maintenance') NOT NULL DEFAULT 'available',
    individual_notes TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (title_id) REFERENCES titles(id) ON DELETE CASCADE,
    UNIQUE KEY unique_title_copy (title_id, copy_number),
    INDEX idx_barcode (barcode),
    INDEX idx_title_id (title_id),
    INDEX idx_loan_status (loan_status)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
