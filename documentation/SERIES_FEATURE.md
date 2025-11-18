# Series Feature Documentation

## Overview

The series feature has been fully implemented to manage book series (collections) in your library. Series support a **one-to-many relationship** with titles, meaning:
- One series can contain multiple titles
- Each title can belong to at most one series (or no series)
- Series can have optional descriptions

## Use Cases

Series are useful for organizing collections such as:
- **Comic series**: Asterix, Tintin, Calvin and Hobbes
- **Book series**: Harry Potter, Lord of the Rings, Foundation
- **Educational series**: "For Dummies" series, technical book series
- **Magazine collections**: Time Magazine, National Geographic
- **Multi-volume works**: Encyclopedia sets, collected works

## Database Schema

### Series Table

```sql
CREATE TABLE series (
    id CHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_name (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
```

### Titles Table (Series Fields)

The titles table includes series relationship fields:

```sql
ALTER TABLE titles
ADD COLUMN series_id CHAR(36),
ADD COLUMN series_number VARCHAR(50);

ALTER TABLE titles
ADD CONSTRAINT fk_titles_series
FOREIGN KEY (series_id) REFERENCES series(id)
ON DELETE SET NULL;
```

**Field Descriptions:**
- `series_id`: Foreign key to the series table (nullable)
- `series_number`: Flexible text field for series position (not used in current implementation)

## Data Model

### Series Structure

```rust
pub struct Series {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Series with Title Count

```rust
pub struct SeriesWithTitleCount {
    pub series: Series,
    pub title_count: i32,
}
```

## API Endpoints

### List All Series

**Endpoint:** `GET /api/v1/series`

**Response:**
```json
[
  {
    "series": {
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "name": "Asterix",
      "description": "French comic book series about Gaulish warriors",
      "created_at": "2024-11-18T10:30:00Z",
      "updated_at": "2024-11-18T10:30:00Z"
    },
    "title_count": 38
  }
]
```

### Get Series by ID

**Endpoint:** `GET /api/v1/series/{id}`

**Response:** Single series object (200 OK) or 404 Not Found

### Create New Series

**Endpoint:** `POST /api/v1/series`

**Request Body:**
```json
{
  "name": "Harry Potter",
  "description": "Fantasy series by J.K. Rowling"
}
```

**Response:** 201 Created with series ID

### Update Series

**Endpoint:** `PUT /api/v1/series/{id}`

**Request Body:**
```json
{
  "name": "Updated Name",
  "description": "Updated description"
}
```

**Response:** 200 OK or 404 Not Found

### Delete Series

**Endpoint:** `DELETE /api/v1/series/{id}`

**Business Rules:**
- ✅ **Success (200 OK)**: Series deleted if no titles are associated
- ❌ **Conflict (409)**: Cannot delete series with associated titles

**Conflict Response:**
```json
{
  "error": {
    "code": "HAS_TITLES",
    "message": "Cannot delete series with associated titles",
    "details": {
      "title_count": 7
    }
  }
}
```

## Frontend Implementation

### Series Management Page

**Location:** `frontend/ui/pages/series_page.slint`

**Features:**
- List all series with title counts
- Create new series via modal dialog
- Edit existing series
- Delete series (with protection for series with titles)
- Delete confirmation dialog
- Empty state message when no series exist
- Alternating row colors for better readability

**Navigation:** Series page is accessible via the "Series" menu item in the sidebar

### Series Selection in Titles

**Location:** `frontend/ui/pages/titles_page.slint`

**Integration:**
- Series dropdown in title create dialog
- Series dropdown in title edit dialog
- Series name displayed in title list (purple color)
- Series dropdown auto-populated from series list
- Optional series association (titles can have no series)

## User Workflows

### Creating a New Series

1. Navigate to **Series** page from sidebar
2. Click **"+ New Series"** button
3. Enter series name (required)
4. Optionally enter description
5. Click **"Save"**
6. Series appears in the list with title count of 0

### Assigning a Title to a Series

1. Navigate to **Titles** page
2. Click **"Edit"** on a title (or create new title)
3. Select series from the **"Series:"** dropdown
4. Click **"Save"**
5. Series name appears in the title list in purple

### Removing a Title from a Series

1. Navigate to **Titles** page
2. Click **"Edit"** on the title
3. Select no series from the dropdown (first empty option)
4. Click **"Save"**

### Deleting a Series

1. Navigate to **Series** page
2. Click **"Delete"** on a series with 0 titles
3. Confirm deletion in the dialog
4. Series is removed

**Note:** Series with associated titles cannot be deleted. Remove titles from the series first.

## Implementation Details

### Database Migrations

Two migrations were created:

1. **`20241118000001_create_series_table.up.sql`**
   - Creates series table with id, name, description
   - Adds index on name for faster lookups

2. **`20241118000002_add_series_to_titles.up.sql`**
   - Adds series_id and series_number columns to titles table
   - Creates foreign key constraint with SET NULL on delete

### Backend Handlers

**File:** `backend/src/handlers/series.rs`

- `list_series()` - Lists all series with title counts via LEFT JOIN
- `get_series()` - Gets single series by ID
- `create_series()` - Creates new series
- `update_series()` - Updates series information
- `delete_series()` - Deletes series with business rule enforcement

### Frontend Components

**Models:** `frontend/src/models.rs`
- Series, SeriesWithTitleCount structs
- CreateSeriesRequest, UpdateSeriesRequest structs

**API Client:** `frontend/src/api_client.rs`
- `get_series()` - Fetches all series
- `create_series()` - Creates new series
- `update_series()` - Updates series
- `delete_series()` - Deletes series

**UI Components:**
- SeriesPage - Full CRUD interface
- SeriesItem - Dropdown item structure
- Series dropdown in title forms

## Statistics

Series statistics are available in the series list:
- **Title Count**: Number of titles in each series
- Displayed next to series name in the list
- Used for delete protection logic

## Future Enhancements

Potential improvements for future versions:

1. **Series Number Display**: Re-enable series_number field for displaying position in series
2. **Series Statistics**: Dedicated statistics page showing:
   - Most popular series (by volume count)
   - Completion tracking (owned vs total titles in series)
3. **Series Import**: Bulk import series from external sources
4. **Series Covers**: Cover images for series
5. **Series Ordering**: Custom sort order for titles within a series
6. **Series Metadata**: Additional fields like publisher, year range, status (ongoing/complete)

## Technical Notes

- **UUID Format**: Series IDs use CHAR(36) format for consistency
- **Cascade Behavior**: When series is deleted, titles' series_id is set to NULL (not cascading delete)
- **Index Strategy**: Name column is indexed for fast searches
- **Character Set**: UTF-8mb4 for international character support
- **Delete Protection**: Enforced at backend level with HTTP 409 Conflict response

## Testing

To test the series feature:

1. **Create series**: Go to Series page, create "Test Series"
2. **Assign to title**: Edit a title, select "Test Series"
3. **Verify display**: Check that series name appears in title list
4. **Test protection**: Try to delete series (should fail with 409)
5. **Remove association**: Edit title, remove series
6. **Delete series**: Now deletion should succeed

## Migration Commands

```bash
cd backend

# Apply migrations
sqlx migrate run

# Revert migrations (if needed)
sqlx migrate revert
```

## Summary

The series feature provides a simple yet powerful way to organize related titles in your library. It follows the same patterns as other entities (genres, publishers) for consistency, includes proper delete protection, and integrates seamlessly with the title management workflow.
