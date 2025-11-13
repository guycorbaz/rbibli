-- Remove address fields from borrowers table
ALTER TABLE borrowers
DROP COLUMN address,
DROP COLUMN city,
DROP COLUMN zip;
