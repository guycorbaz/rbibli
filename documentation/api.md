# Backend API Documentation

## Overview

The backend is a REST API built with **actix-web** and **tokio** for async operations. It provides endpoints for managing the library data. The API is consumed by the Slint-based web frontend (compiled to WebAssembly).

## Current Status

The backend is currently in early development stage. The basic structure is in place with actix-web and tokio configured.

## Architecture

- **Framework**: actix-web 4.11.0
- **Async Runtime**: tokio 1.47.1 (with full features)
- **Language**: Rust (edition 2024)

## Planned Endpoints

### Health Check

```
GET /health
```

Health check endpoint to verify the backend is running. Used by deployment tools (Kubernetes, Docker, etc.) to monitor service health.

**Response:**
- 200 OK: Service is healthy

### Titles Management

```
GET    /api/v1/titles              - List all titles with volume counts
POST   /api/v1/titles              - Create a new title
GET    /api/v1/titles/{id}         - Get title details with volumes
PUT    /api/v1/titles/{id}         - Update title information
DELETE /api/v1/titles/{id}         - Delete a title (if no volumes are loaned)
GET    /api/v1/titles/wishlist     - Get titles with 0 volumes (wishlist)
```

### Volume Management

```
POST   /api/v1/titles/{id}/volumes - Add a new volume to a title
GET    /api/v1/volumes/{id}        - Get volume details
PUT    /api/v1/volumes/{id}        - Update volume information
DELETE /api/v1/volumes/{id}        - Delete a volume (if not loaned)
```

### Barcode Operations

```
GET    /api/v1/scan/volume/{barcode} - Find volume by barcode (Code 128)
GET    /api/v1/scan/isbn/{isbn}      - Find title by ISBN (EAN-13)
POST   /api/v1/scan/loan             - Create loan via barcode scan
POST   /api/v1/scan/return           - Return volume via barcode scan
```

### Loan Management

```
POST   /api/v1/loans                - Create a new loan
GET    /api/v1/loans                - List all loans
GET    /api/v1/loans/active         - Get active loans only
GET    /api/v1/loans/overdue        - Get overdue loans
PUT    /api/v1/loans/{id}/return    - Mark loan as returned
PUT    /api/v1/loans/{id}/extend    - Extend loan due date
```

### Borrower Management

```
GET    /api/v1/borrowers            - List all borrowers
POST   /api/v1/borrowers            - Create a new borrower
GET    /api/v1/borrowers/{id}       - Get borrower details
PUT    /api/v1/borrowers/{id}       - Update borrower information
DELETE /api/v1/borrowers/{id}       - Delete a borrower
```

### Search

```
GET    /api/v1/search/titles        - Search titles by keyword
GET    /api/v1/search/volumes       - Search volumes by various criteria
GET    /api/v1/search/authors       - Search authors by name
```

### Statistics

```
GET    /api/v1/stats/overview       - Get dashboard statistics
GET    /api/v1/stats/loans          - Get loan statistics
GET    /api/v1/stats/collection     - Get collection statistics
```

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
