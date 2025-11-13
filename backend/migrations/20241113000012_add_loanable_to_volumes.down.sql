-- Remove loanable flag from volumes table

ALTER TABLE volumes DROP INDEX idx_volumes_loanable;
ALTER TABLE volumes DROP COLUMN loanable;
