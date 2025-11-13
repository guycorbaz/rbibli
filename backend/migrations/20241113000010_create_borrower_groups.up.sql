-- Create borrower_groups table
-- Groups define loan policies (e.g., Regular, Premium, Staff)

CREATE TABLE borrower_groups (
    id CHAR(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    loan_duration_days INT NOT NULL DEFAULT 21,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_borrower_groups_name (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Insert default groups
INSERT INTO borrower_groups (id, name, loan_duration_days, description) VALUES
(UUID(), 'Regular', 21, 'Standard borrowers with 21-day loan period'),
(UUID(), 'Premium', 42, 'Premium members with extended 42-day loan period'),
(UUID(), 'Staff', 90, 'Library staff with 90-day loan period'),
(UUID(), 'Student', 14, 'Students with 14-day loan period');
