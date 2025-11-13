-- Add address fields to borrowers table
ALTER TABLE borrowers
ADD COLUMN address VARCHAR(255),
ADD COLUMN city VARCHAR(100),
ADD COLUMN zip VARCHAR(20);
