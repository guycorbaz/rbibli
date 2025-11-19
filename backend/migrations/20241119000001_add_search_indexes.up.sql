-- Add indexes for optimized title search and filtering
-- These indexes support the advanced search functionality
-- Note: Foreign key indexes (series_id, genre_id, publisher_id, location_id, title_id in junctions)
-- are automatically created by MySQL/MariaDB and are skipped here

-- Index for title text searches (most common search field)
CREATE INDEX idx_titles_title ON titles(title);

-- Index for ISBN searches (unique lookups)
CREATE INDEX idx_titles_isbn ON titles(isbn);

-- Index for publication year range queries
CREATE INDEX idx_titles_publication_year ON titles(publication_year);

-- Index for language filtering
CREATE INDEX idx_titles_language ON titles(language);

-- Index for Dewey classification searches (prefix matching)
CREATE INDEX idx_titles_dewey_code ON titles(dewey_code);

-- Index for subtitle searches (limited to 255 chars for index)
CREATE INDEX idx_titles_subtitle ON titles(subtitle(255));

-- Index for created_at sorting
CREATE INDEX idx_titles_created_at ON titles(created_at);

-- Index for volume loan status (availability filtering)
CREATE INDEX idx_volumes_loan_status ON volumes(loan_status);

-- Composite index for volume title and availability (title_id likely has FK index, but composite is useful)
CREATE INDEX idx_volumes_title_status ON volumes(title_id, loan_status);

-- Index for author name searches (for free text search across author names)
CREATE INDEX idx_authors_last_name ON authors(last_name);
CREATE INDEX idx_authors_first_name ON authors(first_name);

-- Note: Indexes skipped because they're automatically created by foreign key constraints:
-- - idx_titles_series_id (FK from titles to series)
-- - idx_titles_genre_id (FK from titles to genres)
-- - idx_titles_publisher_id (FK from titles to publishers)
-- - idx_title_authors_title_id (FK from title_authors to titles)
-- - idx_title_authors_author_id (FK from title_authors to authors)
-- - idx_volumes_location_id (FK from volumes to locations)
-- - idx_volumes_title_id (FK from volumes to titles)
