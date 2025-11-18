-- Add extension_count column to loans table to track how many times a loan has been extended
ALTER TABLE loans
ADD COLUMN extension_count INT NOT NULL DEFAULT 0
AFTER due_date;

-- Add index for filtering by extension_count if needed
CREATE INDEX idx_extension_count ON loans(extension_count);
