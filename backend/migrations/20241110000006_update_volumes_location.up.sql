-- Update volumes table to use location_id instead of location text field

-- Add new location_id column
ALTER TABLE volumes ADD COLUMN location_id CHAR(36) AFTER `condition`;

-- Add foreign key constraint
ALTER TABLE volumes ADD CONSTRAINT fk_volumes_location
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE SET NULL;

-- Add index for location_id
ALTER TABLE volumes ADD INDEX idx_location_id (location_id);

-- Drop the old location column (after data migration if needed)
ALTER TABLE volumes DROP COLUMN location;
