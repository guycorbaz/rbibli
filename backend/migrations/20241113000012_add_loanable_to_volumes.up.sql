-- Add loanable flag to volumes table
-- Damaged volumes are consultation-only (not loanable)

ALTER TABLE volumes
ADD COLUMN loanable BOOLEAN NOT NULL DEFAULT TRUE;

-- Add index for loanable volumes queries
CREATE INDEX idx_volumes_loanable ON volumes(loanable);

-- Set damaged volumes as not loanable
UPDATE volumes
SET loanable = FALSE
WHERE `condition` = 'damaged';
