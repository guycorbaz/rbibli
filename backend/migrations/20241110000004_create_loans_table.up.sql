-- Create loans table
CREATE TABLE loans (
    id CHAR(36) PRIMARY KEY,
    title_id CHAR(36) NOT NULL,
    volume_id CHAR(36) NOT NULL,
    borrower_id CHAR(36) NOT NULL,
    loan_date DATETIME NOT NULL,
    due_date DATETIME NOT NULL,
    return_date DATETIME,
    status ENUM('active', 'returned', 'overdue') NOT NULL DEFAULT 'active',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (title_id) REFERENCES titles(id) ON DELETE RESTRICT,
    FOREIGN KEY (volume_id) REFERENCES volumes(id) ON DELETE RESTRICT,
    FOREIGN KEY (borrower_id) REFERENCES borrowers(id) ON DELETE RESTRICT,
    INDEX idx_volume_id (volume_id),
    INDEX idx_borrower_id (borrower_id),
    INDEX idx_status (status),
    INDEX idx_due_date (due_date),
    INDEX idx_loan_date (loan_date)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
