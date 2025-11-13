# Backend API Documentation

## Overview

The backend is a REST API built with **actix-web** and **tokio** for async operations. It provides endpoints for managing the library data. The API is consumed by the Slint-based frontend (currently native desktop, WASM compilation planned for later).

## Current Status (Updated: 2025-01-13)

**Phase 2-3: Actively Developed (~60% Complete)**

The backend has a solid foundation with MariaDB integration and comprehensive CRUD operations for core entities. The database schema is complete for all planned features, but Volume and Loan management handlers are not yet implemented.

### ✅ Fully Implemented
- Health check endpoints (/health, /health/db)
- Titles API (GET, POST, PUT - DELETE missing)
- Authors API (full CRUD)
- Publishers API (full CRUD)
- Genres API (full CRUD)
- Locations API (full CRUD with hierarchical paths)
- Database integration with connection pooling
- UUID-based entity IDs
- Timestamp management

### 🔄 Database Ready, Handlers Needed
- Volume management (table created, no endpoints)
- Borrower management (table created, no endpoints)
- Loan management (table created, no endpoints)
- Title-Author relationships (junction table created, no endpoints)

### ⏳ Not Yet Implemented
- Barcode scanning endpoints
- Search and filter endpoints
- Statistics endpoints
- Import/export endpoints

## Architecture

- **Framework**: actix-web 4.11.0
- **Async Runtime**: tokio 1.47.1 (with full features)
- **Database**: MariaDB via SQLx 0.8.6 (compile-time checked queries)
- **Connection Pooling**: MySqlPoolOptions (max 5 connections)
- **Language**: Rust (edition 2024)

## Implemented Endpoints

### Health Check ✅

```
GET /health          - Basic health check
GET /health/db       - Database connectivity check
```

Health check endpoints to verify the backend and database are running. Used by monitoring tools and deployment systems.

**Responses:**
- 200 OK: Service is healthy
- 500 Internal Server Error: Service or database is down

### Titles Management ✅ (DELETE missing)

```
GET    /api/v1/titles              - ✅ List all titles with volume counts
POST   /api/v1/titles              - ✅ Create a new title
GET    /api/v1/titles/{id}         - ✅ Get title details
PUT    /api/v1/titles/{id}         - ✅ Update title information (partial updates supported)
DELETE /api/v1/titles/{id}         - ⏳ NOT IMPLEMENTED (planned)
GET    /api/v1/titles/wishlist     - ⏳ NOT IMPLEMENTED (can filter volume_count=0)
```

**Implemented Features:**
- LEFT JOIN with volumes to include volume_count in list
- Genre and publisher foreign key relationships
- Partial updates (only changed fields are updated)
- UUID-based IDs
- Created/updated timestamps

### Authors Management ✅ (Full CRUD)

```
GET    /api/v1/authors             - ✅ List all authors with title counts
GET    /api/v1/authors/{id}        - ✅ Get author details
POST   /api/v1/authors             - ✅ Create a new author
PUT    /api/v1/authors/{id}        - ✅ Update author information
DELETE /api/v1/authors/{id}        - ✅ Delete an author
```

**Features:**
- Title count for each author via LEFT JOIN
- Biographical information (birth/death dates, nationality, website)
- UUID-based IDs

### Publishers Management ✅ (Full CRUD)

```
GET    /api/v1/publishers          - ✅ List all publishers with title counts
GET    /api/v1/publishers/{id}     - ✅ Get publisher details
POST   /api/v1/publishers          - ✅ Create a new publisher
PUT    /api/v1/publishers/{id}     - ✅ Update publisher information
DELETE /api/v1/publishers/{id}     - ✅ Delete a publisher
```

**Features:**
- Title count for each publisher
- Company details (founded year, country, website)
- UUID-based IDs

### Genres Management ✅ (Full CRUD)

```
GET    /api/v1/genres              - ✅ List all genres with title counts
GET    /api/v1/genres/{id}         - ✅ Get genre details
POST   /api/v1/genres              - ✅ Create a new genre
PUT    /api/v1/genres/{id}         - ✅ Update genre information
DELETE /api/v1/genres/{id}         - ✅ Delete a genre
```

**Features:**
- Title count for each genre
- Name and description
- UUID-based IDs

### Locations Management ✅ (Full CRUD with Hierarchy)

```
GET    /api/v1/locations           - ✅ List all locations with full hierarchical paths
GET    /api/v1/locations/{id}      - ✅ Get location details
POST   /api/v1/locations           - ✅ Create a new location
PUT    /api/v1/locations/{id}      - ✅ Update location information
DELETE /api/v1/locations/{id}      - ✅ Delete a location
```

**Features:**
- Recursive CTE to build full paths ("Office > Shelf A > Shelf 1")
- Self-referencing hierarchy (parent_id foreign key)
- Volume count per location
- UUID-based IDs

---

## Planned Endpoints (Not Yet Implemented)

### Volume Management ⏳ (CRITICAL - Database Ready)

**Status:** Database table fully created with all fields, handlers needed.

```
POST   /api/v1/titles/{id}/volumes - ⏳ Add a new volume to a title
GET    /api/v1/volumes             - ⏳ List all volumes
GET    /api/v1/volumes/{id}        - ⏳ Get volume details
PUT    /api/v1/volumes/{id}        - ⏳ Update volume information
DELETE /api/v1/volumes/{id}        - ⏳ Delete a volume (if not loaned)
```

**Database Schema Ready:**
- barcode (unique, Code 128 format: VOL-000001)
- copy_number (unique per title)
- condition enum (excellent/good/fair/poor/damaged)
- loan_status enum (available/loaned/overdue/lost/maintenance)
- location_id (FK to locations, SET NULL on delete)
- title_id (FK to titles, CASCADE on delete)
- individual_notes

### Barcode Operations ⏳ (Not Started)

```
GET    /api/v1/scan/volume/{barcode} - ⏳ Find volume by barcode (Code 128)
GET    /api/v1/scan/isbn/{isbn}      - ⏳ Find title by ISBN (EAN-13)
POST   /api/v1/scan/loan             - ⏳ Create loan via barcode scan
POST   /api/v1/scan/return           - ⏳ Return volume via barcode scan
```

### Loan Management ⏳ (CRITICAL - Database Ready)

**Status:** Database table fully created, handlers needed.

```
POST   /api/v1/loans                - ⏳ Create a new loan
GET    /api/v1/loans                - ⏳ List all loans
GET    /api/v1/loans/active         - ⏳ Get active loans only
GET    /api/v1/loans/overdue        - ⏳ Get overdue loans
PUT    /api/v1/loans/{id}/return    - ⏳ Mark loan as returned
PUT    /api/v1/loans/{id}/extend    - ⏳ Extend loan due date (optional)
```

**Database Schema Ready:**
- title_id, volume_id, borrower_id (all FKs with RESTRICT on delete)
- loan_date, due_date, return_date
- status enum (active/returned/overdue)

### Borrower Management ⏳ (Database Ready)

**Status:** Database table created, handlers needed.

```
GET    /api/v1/borrowers            - ⏳ List all borrowers
POST   /api/v1/borrowers            - ⏳ Create a new borrower
GET    /api/v1/borrowers/{id}       - ⏳ Get borrower details
PUT    /api/v1/borrowers/{id}       - ⏳ Update borrower information
DELETE /api/v1/borrowers/{id}       - ⏳ Delete a borrower
```

**Database Schema Ready:**
- name, email, phone
- Simple contact info for trust-based system

### Title-Author Relationships ⏳ (Database Ready)

**Status:** Junction table created with role support, handlers needed.

```
POST   /api/v1/titles/{id}/authors  - ⏳ Add author to title
DELETE /api/v1/titles/{title_id}/authors/{author_id} - ⏳ Remove author from title
PUT    /api/v1/titles/{title_id}/authors/{author_id} - ⏳ Update role/order
```

**Database Schema Ready:**
- title_id, author_id (many-to-many junction)
- role enum (main_author/co_author/translator/illustrator/editor)
- display_order (for author display sequence)

### Search ⏳ (Not Started)

```
GET    /api/v1/search/titles        - ⏳ Search titles by keyword
GET    /api/v1/search/volumes       - ⏳ Search volumes by various criteria
GET    /api/v1/search/authors       - ⏳ Search authors by name
```

**Future Features:**
- Full-text search in title and summary fields
- Filter by genre, publisher, author, location
- Filter by availability, condition
- Sort options (title, year, recently added)

### Statistics ⏳ (Not Started)

```
GET    /api/v1/stats/overview       - ⏳ Get dashboard statistics
GET    /api/v1/stats/loans          - ⏳ Get loan statistics
GET    /api/v1/stats/collection     - ⏳ Get collection statistics
```

**Future Features:**
- Total titles/volumes/borrowers count
- Active/overdue loans count
- Most loaned titles
- Collection growth over time
- Borrower activity

## Data Models

### Title
```json
{
  "id": "uuid",
  "title": "string",
  "subtitle": "string?",
  "isbn": "string?",
  "publisher": "string?",
  "publication_year": "number?",
  "pages": "number?",
  "language": "string",
  "dewey_code": "string?",
  "dewey_category": "string?",
  "genre": "string?",
  "summary": "string?",
  "cover_url": "string?",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Volume
```json
{
  "id": "uuid",
  "title_id": "uuid",
  "copy_number": "number",
  "barcode": "string",
  "condition": "excellent|good|fair|poor|damaged",
  "location": "string?",
  "loan_status": "available|loaned|overdue|lost|maintenance",
  "individual_notes": "string?",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Loan
```json
{
  "id": "uuid",
  "title_id": "uuid",
  "volume_id": "uuid",
  "borrower_id": "uuid",
  "loan_date": "datetime",
  "due_date": "datetime",
  "return_date": "datetime?",
  "status": "active|returned|overdue",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Borrower
```json
{
  "id": "uuid",
  "name": "string",
  "email": "string?",
  "phone": "string?",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

## Error Responses

All endpoints follow standard HTTP status codes:

- **200 OK**: Successful request
- **201 Created**: Resource successfully created
- **400 Bad Request**: Invalid request data
- **404 Not Found**: Resource not found
- **409 Conflict**: Resource conflict (e.g., duplicate barcode)
- **500 Internal Server Error**: Server error

Error response format:
```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Human-readable error message",
    "details": {}
  }
}
```

## Database Integration (Planned)

The backend uses **MariaDB** for data persistence:
- **MariaDB**: Production-grade, MySQL-compatible database
- **SQLx**: Compile-time checked queries with async support
- **Connection pooling**: Efficient connection management
- **Migrations**: Version-controlled schema changes via sqlx-cli

Database abstraction is handled through the repository pattern with trait-based interfaces.

## Authentication (Planned)

For personal use, authentication will be optional and simple:
- Basic username/password authentication
- Session-based (no complex JWT for personal use)
- Guest read-only access option

## Running the Backend

```bash
cd backend
cargo run
```

The server will start on `http://localhost:8000` by default.

## Testing

```bash
cd backend
cargo test
```

## Development

See `development_environment.md` for setup instructions and `CLAUDE.md` for architecture overview.
