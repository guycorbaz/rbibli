-- Remove group_id from borrowers table

ALTER TABLE borrowers DROP FOREIGN KEY fk_borrowers_group;
ALTER TABLE borrowers DROP INDEX idx_borrowers_group_id;
ALTER TABLE borrowers DROP COLUMN group_id;
