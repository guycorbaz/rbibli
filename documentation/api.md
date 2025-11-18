# Backend API Documentation

## Overview

The backend is a REST API built with **actix-web** and **tokio** for async operations. It provides endpoints for managing the library data. The API is consumed by the Slint-based frontend (currently native desktop, WASM compilation planned for later).

## Current Status (Updated: 2024-11-15)

**Phase 3: Nearly Complete (~85% Complete)**

The backend has comprehensive functionality with MariaDB integration and full CRUD operations for all core entities. Volume management, loan system, statistics dashboard, ISBN lookup, Dewey classification, and cover image uploads are all fully implemented.

### ✅ Fully Implemented
- Health check endpoints (/health, /health/db)
- **Titles API** (full CRUD with business rule validation)
- **Volumes API** (full CRUD with barcode support)
- **Authors API** (full CRUD)
- **Publishers API** (full CRUD)
- **Genres API** (full CRUD)
- **Series API** (full CRUD with title associations)
- **Locations API** (full CRUD with hierarchical paths)
- **Borrowers API** (full CRUD with group association)
- **Borrower Groups API** (full CRUD with loan policies)
- **Loans API** (create by barcode, list active/overdue, return)
- **Statistics API** (library overview, volumes per genre/location, loan status)
- **ISBN Lookup API** (Google Books integration)
- **Dewey Classification API** (search, browse, get by code)
- **Cover Upload API** (upload, get, delete cover images)
- Database integration with connection pooling
- UUID-based entity IDs
- Timestamp management (created_at, updated_at)

### ⏳ Planned / Not Yet Implemented
- Title-Author relationship endpoints (junction table exists)
- Loan extension functionality
- Advanced search and filter endpoints
- Import/export endpoints (CSV, JSON)
- Barcode generation endpoints

## Architecture

- **Framework**: actix-web 4.11.0
- **Async Runtime**: tokio 1.47.1 (with full features)
- **Database**: MariaDB via SQLx 0.8.6 (compile-time checked queries)
- **Connection Pooling**: MySqlPoolOptions (max 5 connections)
- **Language**: Rust (edition 2024)

## Base URL

```
http://localhost:8000
```

---

## Implemented Endpoints

### Health Check ✅

Check service and database health status.

```
GET /health          - Basic health check
GET /health/db       - Database connectivity check
```

**Responses:**
- `200 OK`: Service is healthy
- `503 Service Unavailable`: Database connection failed

**Example Response** (`/health/db`):
```json
{
  "status": "ok",
  "database": "connected"
}
```

---

### Titles Management ✅

Manage book titles (abstract book metadata).

```
GET    /api/v1/titles              - List all titles with volume counts
POST   /api/v1/titles              - Create a new title
PUT    /api/v1/titles/{id}         - Update title information (partial updates)
DELETE /api/v1/titles/{id}         - Delete a title (only if no volumes exist)
```

**Features:**
- LEFT JOIN with volumes to include `volume_count` in listings
- Genre, publisher, and series foreign key relationships
- Series association with optional series_number field
- Partial updates (only changed fields are updated)
- **Business rule enforcement**: Titles with volumes cannot be deleted
- ISBN, Dewey classification, cover URL support

**DELETE Business Rules:**
- **Success (200)**: Title deleted if `volume_count == 0`
- **Not Found (404)**: Title ID doesn't exist
- **Conflict (409)**: Title has volumes, returns:
  ```json
  {
    "error": {
      "code": "HAS_VOLUMES",
      "message": "Cannot delete title with existing volumes",
      "details": { "volume_count": 3 }
    }
  }
  ```

**Example Title Object:**
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "title": "The Rust Programming Language",
  "subtitle": "2nd Edition",
  "isbn": "9781718500440",
  "publisher": "No Starch Press",
  "publisher_id": "pub-uuid",
  "publication_year": 2023,
  "pages": 560,
  "language": "en",
  "dewey_code": "005.133",
  "dewey_category": "Computer programming",
  "genre": "Programming",
  "genre_id": "genre-uuid",
  "series_name": "Programming Series",
  "series_id": "series-uuid",
  "series_number": "",
  "summary": "Learn Rust programming...",
  "cover_url": "https://...",
  "volume_count": 2,
  "created_at": 1699564800,
  "updated_at": 1699564800
}
```

---

### Volumes Management ✅

Manage physical book copies with barcode tracking.

```
GET    /api/v1/titles/{title_id}/volumes  - List volumes for a specific title
POST   /api/v1/volumes                    - Create a new volume
GET    /api/v1/volumes/{id}               - Get volume details
PUT    /api/v1/volumes/{id}               - Update volume information
DELETE /api/v1/volumes/{id}               - Delete a volume (if not loaned)
```

**Features:**
- Unique barcode per volume (Code 128 format: `VOL-000001`)
- Automatic copy numbering per title
- Condition tracking (excellent, good, fair, poor, damaged)
- Loan status tracking (available, loaned, overdue, lost, maintenance)
- Location assignment with FK to locations table
- Individual volume notes

**Example Volume Object:**
```json
{
  "id": "vol-uuid",
  "title_id": "title-uuid",
  "copy_number": 1,
  "barcode": "VOL-000001",
  "condition": "good",
  "location_id": "location-uuid",
  "loan_status": "available",
  "individual_notes": "Gift from friend",
  "created_at": 1699564800,
  "updated_at": 1699564800
}
```

---

### Authors Management ✅

Manage book authors with biographical information.

```
GET    /api/v1/authors             - List all authors with title counts
GET    /api/v1/authors/{id}        - Get author details
POST   /api/v1/authors             - Create a new author
PUT    /api/v1/authors/{id}        - Update author information
DELETE /api/v1/authors/{id}        - Delete an author
```

**Features:**
- Title count per author via LEFT JOIN
- Biographical information (birth/death dates, nationality, biography)
- Website and contact information

**Example Author Object:**
```json
{
  "id": "author-uuid",
  "name": "Steve Klabnik",
  "biography": "Technical writer and Rust core team member",
  "birth_date": "1985-06-15",
  "death_date": null,
  "nationality": "American",
  "website": "https://steveklabnik.com",
  "title_count": 5,
  "created_at": 1699564800,
  "updated_at": 1699564800
}
```

---

### Publishers Management ✅

Manage publishing companies and their catalogs.

```
GET    /api/v1/publishers          - List all publishers with title counts
GET    /api/v1/publishers/{id}     - Get publisher details
POST   /api/v1/publishers          - Create a new publisher
PUT    /api/v1/publishers/{id}     - Update publisher information
DELETE /api/v1/publishers/{id}     - Delete a publisher
```

**Features:**
- Title count per publisher
- Company details (founded year, country, website, description)

**Example Publisher Object:**
```json
{
  "id": "pub-uuid",
  "name": "No Starch Press",
  "description": "Publisher of tech books",
  "website_url": "https://nostarch.com",
  "country": "USA",
  "founded_year": 1994,
  "title_count": 12,
  "created_at": 1699564800,
  "updated_at": 1699564800
}
```

---

### Genres Management ✅

Manage book genres and categories.

```
GET    /api/v1/genres              - List all genres with title counts
GET    /api/v1/genres/{id}         - Get genre details
POST   /api/v1/genres              - Create a new genre
PUT    /api/v1/genres/{id}         - Update genre information
DELETE /api/v1/genres/{id}         - Delete a genre
```

**Example Genre Object:**
```json
{
  "id": "genre-uuid",
  "name": "Science Fiction",
  "description": "Speculative fiction based on scientific concepts",
  "title_count": 42,
  "created_at": 1699564800,
  "updated_at": 1699564800
}
```

---

### Series Management ✅

Manage book series collections (e.g., Harry Potter, Asterix, etc.).

```
GET    /api/v1/series              - List all series with title counts
GET    /api/v1/series/{id}         - Get series details
POST   /api/v1/series              - Create a new series
PUT    /api/v1/series/{id}         - Update series information
DELETE /api/v1/series/{id}         - Delete a series (only if no titles associated)
```

**Features:**
- Series-to-title relationship (one-to-many)
- Title count per series
- Delete protection: cannot delete series with associated titles
- Series can have optional description

**Example Series Object:**
```json
{
  "id": "series-uuid",
  "name": "Asterix",
  "description": "French comic book series about Gaulish warriors",
  "title_count": 38,
  "created_at": 1699564800,
  "updated_at": 1699564800
}
```

**DELETE Business Rules:**
- **Success (200)**: Series deleted if `title_count == 0`
- **Conflict (409)**: Series has associated titles, cannot delete

---

### Locations Management ✅

Manage storage locations with hierarchical organization.

```
GET    /api/v1/locations           - List all locations with full hierarchical paths
GET    /api/v1/locations/{id}      - Get location details
POST   /api/v1/locations           - Create a new location
PUT    /api/v1/locations/{id}      - Update location information
DELETE /api/v1/locations/{id}      - Delete a location
```

**Features:**
- Recursive CTE to build full paths: `"Office > Bookshelf A > Shelf 3"`
- Self-referencing hierarchy (parent_id foreign key)
- Volume count per location
- Level tracking (0 = root, 1 = child, etc.)

**Example Location Object:**
```json
{
  "id": "location-uuid",
  "name": "Shelf 3",
  "description": "Top shelf",
  "parent_id": "bookshelf-uuid",
  "full_path": "Office > Bookshelf A > Shelf 3",
  "level": 2,
  "created_at": 1699564800,
  "updated_at": 1699564800
}
```

---

### Borrowers Management ✅

Manage library borrowers (friends, family, colleagues).

```
GET    /api/v1/borrowers            - List all borrowers with group information
POST   /api/v1/borrowers            - Create a new borrower
PUT    /api/v1/borrowers/{id}       - Update borrower information
DELETE /api/v1/borrowers/{id}       - Delete a borrower
```

**Features:**
- Simple contact information (name, email, phone, address)
- Borrower group association for loan policies
- Trust-based system (no complex restrictions)

**Example Borrower Object:**
```json
{
  "id": "borrower-uuid",
  "name": "John Doe",
  "email": "john@example.com",
  "phone": "+1234567890",
  "address": "123 Main St",
  "city": "Portland",
  "zip": "97201",
  "group_id": "friends-group-uuid",
  "group_name": "Friends",
  "loan_duration_days": 21,
  "created_at": 1699564800,
  "updated_at": 1699564800
}
```

---

### Borrower Groups Management ✅

Manage borrower groups with custom loan policies.

```
GET    /api/v1/borrower-groups      - List all borrower groups
POST   /api/v1/borrower-groups      - Create a new borrower group
PUT    /api/v1/borrower-groups/{id} - Update borrower group
DELETE /api/v1/borrower-groups/{id} - Delete a borrower group
```

**Features:**
- Custom loan duration per group (in days)
- Group descriptions and metadata
- Applied automatically when creating loans

**Example Borrower Group Object:**
```json
{
  "id": "group-uuid",
  "name": "Friends",
  "loan_duration_days": 21,
  "description": "Close friends with longer loan periods",
  "created_at": 1699564800,
  "updated_at": 1699564800
}
```

---

### Loans Management ✅

Manage book loans with barcode-based checkout.

```
GET    /api/v1/loans                - List active loans with details
POST   /api/v1/loans                - Create a new loan by barcode
GET    /api/v1/loans/overdue        - List overdue loans
POST   /api/v1/loans/{id}/return    - Return a loaned volume
```

**Features:**
- Barcode-based loan creation
- Automatic due date calculation from borrower group policy
- Overdue status calculation
- Loan history tracking
- Volume status updates on return

**Create Loan Request:**
```json
{
  "borrower_id": "borrower-uuid",
  "barcode": "VOL-000001"
}
```

**Loan Detail Response:**
```json
{
  "loan": {
    "id": "loan-uuid",
    "title_id": "title-uuid",
    "volume_id": "volume-uuid",
    "borrower_id": "borrower-uuid",
    "loan_date": "2024-11-01",
    "due_date": "2024-11-22",
    "returned_at": null
  },
  "title": "The Rust Programming Language",
  "barcode": "VOL-000001",
  "borrower_name": "John Doe",
  "borrower_email": "john@example.com",
  "is_overdue": false
}
```

---

### Statistics ✅

View library analytics and statistics.

```
GET /api/v1/statistics/library      - Overall library statistics
GET /api/v1/statistics/genres       - Volumes per genre
GET /api/v1/statistics/locations    - Volumes per location
GET /api/v1/statistics/loans        - Loan status breakdown
```

**Library Statistics Response:**
```json
{
  "total_titles": 150,
  "total_volumes": 200,
  "total_authors": 75,
  "total_publishers": 20,
  "total_genres": 15,
  "total_locations": 8,
  "total_borrowers": 12,
  "active_loans": 5,
  "overdue_loans": 1
}
```

**Genre Statistics Response:**
```json
[
  {
    "genre_id": "genre-uuid",
    "genre_name": "Science Fiction",
    "title_count": 42,
    "volume_count": 58
  }
]
```

**Location Statistics Response:**
```json
[
  {
    "location_id": "location-uuid",
    "location_name": "Shelf 3",
    "location_path": "Office > Bookshelf A > Shelf 3",
    "volume_count": 25
  }
]
```

---

### ISBN Lookup ✅

Look up book metadata via ISBN using Google Books API.

```
GET /api/v1/isbn/{isbn}             - Lookup book by ISBN
```

**Query Parameters:**
- `isbn` (path) - 10 or 13 digit ISBN

**Response:**
```json
{
  "title": "The Rust Programming Language",
  "subtitle": "2nd Edition",
  "authors": ["Steve Klabnik", "Carol Nichols"],
  "publisher": "No Starch Press",
  "published_date": "2023",
  "description": "Learn Rust programming...",
  "page_count": 560,
  "categories": ["Computers"],
  "language": "en",
  "isbn_10": "1718500440",
  "isbn_13": "9781718500440",
  "cover_url": "https://books.google.com/...",
  "preview_link": "https://books.google.com/..."
}
```

---

### Dewey Classification ✅

Search and browse Dewey Decimal Classification system.

```
GET /api/v1/dewey/search            - Search Dewey classifications
GET /api/v1/dewey/browse            - Browse Dewey hierarchy
GET /api/v1/dewey/{code}            - Get classification by code
```

**Search Query Parameters:**
- `q` - Search query (searches code, name, description)

**Browse Query Parameters:**
- `parent` - Parent code to browse children (omit for top level)

**Example Classification:**
```json
{
  "code": "005.133",
  "name": "Specific programming languages",
  "level": 3,
  "description": "Programming in specific languages like Rust, Python, etc.",
  "relevance": 0.95
}
```

---

### Cover Image Uploads ✅

Upload and manage book cover images.

```
POST   /api/v1/uploads/cover         - Upload cover image
GET    /api/v1/uploads/cover/{title_id} - Get cover image
DELETE /api/v1/uploads/cover/{title_id} - Delete cover image
```

**Upload Request:**
- Content-Type: `multipart/form-data`
- Field: `cover` (image file)
- Supported formats: JPEG, PNG, GIF
- Max size: 5MB

**Response:**
```json
{
  "url": "/api/v1/uploads/cover/title-uuid"
}
```

---

## Error Responses

All endpoints follow standard HTTP status codes:

- **200 OK**: Successful request
- **201 Created**: Resource successfully created
- **400 Bad Request**: Invalid request data
- **404 Not Found**: Resource not found
- **409 Conflict**: Resource conflict (e.g., duplicate barcode, title has volumes)
- **500 Internal Server Error**: Server error

**Error Response Format:**
```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Human-readable error message",
    "details": {}
  }
}
```

**Common Error Codes:**
- `HAS_VOLUMES` - Cannot delete title with existing volumes
- `DUPLICATE_BARCODE` - Volume barcode already exists
- `NOT_FOUND` - Resource not found
- `INVALID_REQUEST` - Invalid request data
- `DATABASE_ERROR` - Database operation failed

---

## Database

The backend uses **MariaDB** for data persistence:

- **MariaDB**: Production-grade, MySQL-compatible database
- **SQLx**: Compile-time checked queries with async support
- **Connection pooling**: MySqlPoolOptions (max 5 connections)
- **Migrations**: 13 migrations applied via sqlx-cli
- **UUID-based IDs**: Using CHAR(36) format
- **Timestamps**: created_at, updated_at on all entities

---

## Running the Backend

### Start the server

```bash
cd backend
cargo run
```

Server starts on `http://localhost:8000` by default.

### Configure via environment variables

Create `backend/.env`:
```env
DATABASE_URL=mysql://username:password@localhost:3306/rbibli
HOST=127.0.0.1
PORT=8000
RUST_LOG=info
```

### Run database migrations

```bash
cd backend
sqlx migrate run
```

---

## Testing

```bash
cd backend
cargo test
```

---

## Development

See [`development_environment.md`](development_environment.md) for detailed setup instructions and [`CLAUDE.md`](CLAUDE.md) for architecture overview.

---

## Future Planned Features

### Title-Author Relationships ⏳
- `POST /api/v1/titles/{id}/authors` - Add author to title with role
- `DELETE /api/v1/titles/{title_id}/authors/{author_id}` - Remove author from title
- `PUT /api/v1/titles/{title_id}/authors/{author_id}` - Update author role/order

### Series Management ⏳
- Full CRUD for series (collections of related titles)
- Series ordering and numbering

### Advanced Features ⏳
- Full-text search across titles and summaries
- Loan extension functionality
- Import/export (CSV, JSON)
- Barcode generation API
- Advanced filtering and sorting options
