-- Rollback: Remove search optimization indexes
-- Note: Foreign key indexes are not dropped as they're managed by FK constraints

-- Drop title indexes
DROP INDEX idx_titles_title ON titles;
DROP INDEX idx_titles_isbn ON titles;
DROP INDEX idx_titles_publication_year ON titles;
DROP INDEX idx_titles_language ON titles;
DROP INDEX idx_titles_dewey_code ON titles;
DROP INDEX idx_titles_subtitle ON titles;
DROP INDEX idx_titles_created_at ON titles;

-- Drop volume indexes
DROP INDEX idx_volumes_loan_status ON volumes;
DROP INDEX idx_volumes_title_status ON volumes;

-- Drop author name indexes
DROP INDEX idx_authors_last_name ON authors;
DROP INDEX idx_authors_first_name ON authors;
