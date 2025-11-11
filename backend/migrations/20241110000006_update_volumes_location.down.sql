-- Revert volumes table changes

-- Add back the old location column
ALTER TABLE volumes ADD COLUMN location VARCHAR(200) AFTER `condition`;

-- Drop the foreign key constraint
ALTER TABLE volumes DROP FOREIGN KEY fk_volumes_location;

-- Drop the location_id column
ALTER TABLE volumes DROP COLUMN location_id;
