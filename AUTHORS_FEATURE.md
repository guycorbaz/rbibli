# Authors Feature Documentation

## Overview

The authors feature has been fully implemented to manage book authors in your library. Authors support a **many-to-many relationship** with titles, meaning:
- One author can write multiple titles
- One title can have multiple authors (with different roles)

## Database Schema

### Authors Table

```sql
CREATE TABLE authors (
    id CHAR(36) PRIMARY KEY,
    first_name VARCHAR(200) NOT NULL,
    last_name VARCHAR(200) NOT NULL,
    biography TEXT,
    birth_date DATE,
    death_date DATE,
    nationality VARCHAR(100),
    website_url VARCHAR(500),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
```

### Title-Authors Junction Table

```sql
CREATE TABLE title_authors (
    id CHAR(36) PRIMARY KEY,
    title_id CHAR(36) NOT NULL,
    author_id CHAR(36) NOT NULL,
    role ENUM('main_author', 'co_author', 'translator', 'illustrator', 'editor'),
    display_order INT NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (title_id) REFERENCES titles(id) ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES authors(id) ON DELETE CASCADE
);
```

## Author Roles

Authors can have different roles in a title:

| Role | Description |
|------|-------------|
| **main_author** | Primary author of the work |
| **co_author** | Co-author or collaborator |
| **translator** | Translated the work to another language |
| **illustrator** | Created illustrations for the work |
| **editor** | Edited the work |

## Many-to-Many Relationship

### Example: Title with Multiple Authors

```
Title: "The Evolution of Cooperation"
├── Robert Axelrod (main_author)
└── William Hamilton (co_author)

Title: "Foundation"
└── Isaac Asimov (main_author)
```

### Example: Author with Multiple Titles

```
Author: Isaac Asimov
├── "Foundation" (main_author)
├── "I, Robot" (main_author)
└── "The Gods Themselves" (main_author)
```

## API Endpoints

All endpoints are under `/api/v1/authors`

### 1. List All Authors

```
GET /api/v1/authors
```

Returns all authors with the count of titles they've authored.

**Response:**
```json
[
  {
    "id": "uuid",
    "first_name": "Isaac",
    "last_name": "Asimov",
    "biography": "American science fiction writer",
    "birth_date": "1920-01-02",
    "death_date": "1992-04-06",
    "nationality": "American",
    "website_url": null,
    "created_at": 1699564800,
    "updated_at": 1699564800,
    "title_count": 15
  }
]
```

### 2. Get Single Author

```
GET /api/v1/authors/{id}
```

Get details of a specific author by UUID.

**Response:**
```json
{
  "id": "uuid",
  "first_name": "Isaac",
  "last_name": "Asimov",
  "biography": "American science fiction writer and professor of biochemistry",
  "birth_date": "1920-01-02",
  "death_date": "1992-04-06",
  "nationality": "American",
  "website_url": "https://www.asimovonline.com",
  "created_at": 1699564800,
  "updated_at": 1699564800
}
```

### 3. Create Author

```
POST /api/v1/authors
Content-Type: application/json

{
  "first_name": "Isaac",
  "last_name": "Asimov",
  "biography": "American science fiction writer",
  "birth_date": "1920-01-02",
  "death_date": "1992-04-06",
  "nationality": "American",
  "website_url": "https://www.asimovonline.com"
}
```

**Response:**
```json
{
  "id": "new-uuid",
  "message": "Author created successfully"
}
```

### 4. Update Author

```
PUT /api/v1/authors/{id}
Content-Type: application/json

{
  "first_name": "Isaac",
  "biography": "Updated biography",
  "website_url": "https://new-website.com"
}
```

All fields are optional. Only provided fields will be updated.

**Response:**
```json
{
  "message": "Author updated successfully"
}
```

### 5. Delete Author

```
DELETE /api/v1/authors/{id}
```

Deletes an author. The associated title-author relationships will also be deleted (CASCADE).

**Response:**
```json
{
  "message": "Author deleted successfully"
}
```

## Data Model

### Author Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | UUID | Auto | Unique identifier |
| `first_name` | String | Yes | Author's first name |
| `last_name` | String | Yes | Author's last name |
| `biography` | Text | No | Author biography |
| `birth_date` | Date | No | Birth date (YYYY-MM-DD) |
| `death_date` | Date | No | Death date (YYYY-MM-DD) |
| `nationality` | String | No | Author's nationality |
| `website_url` | String | No | Author's website |
| `created_at` | DateTime | Auto | Creation timestamp |
| `updated_at` | DateTime | Auto | Last update timestamp |

### Title-Author Relationship Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | UUID | Auto | Unique identifier |
| `title_id` | UUID | Yes | Reference to title |
| `author_id` | UUID | Yes | Reference to author |
| `role` | Enum | Yes | Author's role in this title |
| `display_order` | Integer | Yes | Display order (for sorting) |
| `created_at` | DateTime | Auto | Creation timestamp |

## Example Usage

### Creating Authors

```bash
# Create Isaac Asimov
curl -X POST http://localhost:8000/api/v1/authors \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Isaac",
    "last_name": "Asimov",
    "biography": "American science fiction writer and professor of biochemistry",
    "birth_date": "1920-01-02",
    "death_date": "1992-04-06",
    "nationality": "American"
  }'

# Create Arthur C. Clarke
curl -X POST http://localhost:8000/api/v1/authors \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Arthur",
    "last_name": "Clarke",
    "biography": "British science fiction writer and futurist",
    "birth_date": "1917-12-16",
    "death_date": "2008-03-19",
    "nationality": "British"
  }'
```

### Listing Authors

```bash
curl http://localhost:8000/api/v1/authors | jq '.'
```

Output:
```json
[
  {
    "first_name": "Arthur",
    "last_name": "Clarke",
    "nationality": "British",
    "title_count": 8
  },
  {
    "first_name": "Isaac",
    "last_name": "Asimov",
    "nationality": "American",
    "title_count": 15
  }
]
```

### Updating an Author

```bash
curl -X PUT http://localhost:8000/api/v1/authors/{uuid} \
  -H "Content-Type: application/json" \
  -d '{
    "biography": "Updated biography with more details",
    "website_url": "https://asimovonline.com"
  }'
```

### Deleting an Author

```bash
curl -X DELETE http://localhost:8000/api/v1/authors/{uuid}
```

## Integration with Titles

To associate authors with titles, you'll use the `title_authors` table. Future API endpoints will be added for:

```
POST /api/v1/titles/{title_id}/authors
{
  "author_id": "uuid",
  "role": "main_author",
  "display_order": 1
}

GET /api/v1/titles/{title_id}/authors
DELETE /api/v1/titles/{title_id}/authors/{author_id}
```

These will allow you to:
1. Add an author to a title with a specific role
2. List all authors for a title
3. Remove an author from a title

## Migrations Applied

The migrations have been successfully applied:

```bash
sqlx migrate info
```

Output:
```
20241110000007/installed create authors table
20241110000008/installed create title authors table
```

## Testing the API

### Test with curl

```bash
# 1. Create an author
AUTHOR_ID=$(curl -X POST http://localhost:8000/api/v1/authors \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Test",
    "last_name": "Author",
    "biography": "Test biography"
  }' | jq -r '.id')

# 2. Get the author
curl http://localhost:8000/api/v1/authors/$AUTHOR_ID | jq '.'

# 3. Update the author
curl -X PUT http://localhost:8000/api/v1/authors/$AUTHOR_ID \
  -H "Content-Type: application/json" \
  -d '{
    "biography": "Updated test biography"
  }'

# 4. List all authors
curl http://localhost:8000/api/v1/authors | jq '.'

# 5. Delete the author
curl -X DELETE http://localhost:8000/api/v1/authors/$AUTHOR_ID
```

## Error Responses

All endpoints return standard error responses:

```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Human-readable message",
    "details": {}
  }
}
```

Common error codes:
- `INVALID_UUID`: Malformed author ID
- `NOT_FOUND`: Author doesn't exist
- `DATABASE_ERROR`: Database operation failed
- `NO_UPDATES`: No fields provided for update

## Logging

All author operations are logged:

- **INFO**: API calls, successful operations, counts
- **WARN**: Invalid UUIDs, not found errors
- **ERROR**: Database failures
- **DEBUG**: Query execution, row processing

Enable debug logging:
```bash
RUST_LOG=debug cargo run
```

## Use Cases

### Use Case 1: Single Author Book

```bash
# 1. Create author
ASIMOV_ID=$(curl -X POST http://localhost:8000/api/v1/authors \
  -H "Content-Type: application/json" \
  -d '{"first_name": "Isaac", "last_name": "Asimov"}' \
  | jq -r '.id')

# 2. Create title (future endpoint)
# POST /api/v1/titles
# {
#   "title": "Foundation",
#   "isbn": "9780553293357"
# }

# 3. Associate author with title (future endpoint)
# POST /api/v1/titles/{title_id}/authors
# {
#   "author_id": "{asimov_id}",
#   "role": "main_author"
# }
```

### Use Case 2: Multi-Author Book

```bash
# Create multiple authors
AUTHOR1_ID=$(curl -X POST ... | jq -r '.id')
AUTHOR2_ID=$(curl -X POST ... | jq -r '.id')

# Associate both with the same title with different roles
# POST /api/v1/titles/{title_id}/authors
# [
#   {"author_id": "{author1_id}", "role": "main_author", "display_order": 1},
#   {"author_id": "{author2_id}", "role": "co_author", "display_order": 2}
# ]
```

### Use Case 3: Translator Credit

```bash
# Original author
TOLKIEN_ID=$(curl -X POST ... | jq -r '.id')

# Translator
TRANSLATOR_ID=$(curl -X POST ... | jq -r '.id')

# Associate both with translated version
# POST /api/v1/titles/{title_id}/authors
# [
#   {"author_id": "{tolkien_id}", "role": "main_author"},
#   {"author_id": "{translator_id}", "role": "translator"}
# ]
```

## Next Steps

### Immediate

1. **Test the API** - Use curl to create, list, update, delete authors
2. **Frontend UI** - Add authors management page in Slint
3. **Title-Author Association** - Implement endpoints to link authors to titles

### Future Enhancements

1. **Search** - Search authors by name
2. **Pagination** - Paginate author list
3. **Filtering** - Filter by nationality, birth year, etc.
4. **Sorting** - Sort by name, title count, etc.
5. **Photo/Avatar** - Add author photo URL
6. **Social Links** - Twitter, Wikipedia, etc.
7. **Statistics** - Most prolific authors, popular authors
8. **Import** - Import authors from external APIs
9. **Merge** - Merge duplicate author entries
10. **Validation** - Check for duplicate authors

## Summary

✅ **Complete CRUD** operations for authors
✅ **Many-to-many** relationship with titles
✅ **Role support** (main author, co-author, translator, etc.)
✅ **Comprehensive API** with proper logging
✅ **Database migrations** applied successfully
✅ **Error handling** with meaningful messages

The authors feature is now fully functional and ready to use!
