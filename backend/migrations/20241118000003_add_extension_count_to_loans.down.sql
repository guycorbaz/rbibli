-- Remove extension_count column and index
DROP INDEX idx_extension_count ON loans;

ALTER TABLE loans
DROP COLUMN extension_count;
