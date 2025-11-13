-- Add group_id to borrowers table

ALTER TABLE borrowers
ADD COLUMN group_id CHAR(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- Add foreign key constraint
ALTER TABLE borrowers
ADD CONSTRAINT fk_borrowers_group
FOREIGN KEY (group_id) REFERENCES borrower_groups(id)
ON DELETE SET NULL;

-- Add index for better query performance
CREATE INDEX idx_borrowers_group_id ON borrowers(group_id);

-- Set all existing borrowers to 'Regular' group by default
UPDATE borrowers b
SET b.group_id = (SELECT id FROM borrower_groups WHERE name = 'Regular' LIMIT 1)
WHERE b.group_id IS NULL;
