# Technical Architecture - Personal Library Manager

## Overview

**Web application** developed in Rust with a modular, performant and secure architecture for managing a personal library. The frontend uses the **Slint** UI framework (native-first, WASM-ready), and the backend uses **actix-web** for the REST API.

## 1. Technical Stack

### 1.1 Backend (Rust)

- **Web framework**: actix-web (async, high-performance)
- **Database**: MariaDB with SQLx (async, compile-time checked queries)
- **Async Runtime**: tokio
- **Serialization**: serde (JSON)
- **Logging**: log + env_logger
- **Configuration**: config crate + environment variables

### 1.2 Frontend (Slint)

- **UI Framework**: Slint (Declarative GUI for tracking titles, volumes, loans)
- **Language**: .slint markup + Rust business logic
- **HTTP Client**: reqwest (native) / reqwest-wasm (web)
- **State Management**: Slint global singletons + Rust callbacks
- **Build Targets**: Native Desktop (Linux/Windows/macOS) and WebAssembly (WASM)

### 1.3 Infrastructure

- **Containerization**: Docker (Frontend + Backend)
- **Database**: MariaDB 10.11+
- **File storage**: Local filesystem (for covers)

## 2. General Architecture

### 2.1 Project structure

```
library-manager/
├── backend/
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── configuration.rs
│   │   ├── models/
│   │   ├── handlers/
│   │   └── migrations/
│   ├── Cargo.toml
│   └── static/             # Static files for WASM frontend
├── frontend/
│   ├── src/
│   │   ├── main.rs         # Rust logic & callbacks
│   │   └── ...
│   ├── ui/
│   │   ├── app-window.slint # Main UI definition
│   │   ├── pages/
│   │   │   ├── titles_page.slint
│   │   │   └── ...
│   │   └── side_bar.slint
│   ├── Cargo.toml
│   └── index.html
├── Dockerfile              # Unified build
├── docker-compose.yml
└── documentation/
```

### 2.2 Layered architecture

```mermaid
graph TD
    Client[Client Browser / Desktop App]
    Slint[Slint UI (WASM/Native)]
    Actix[Actix Web REST API]
    SQLx[SQLx Data Access]
    DB[(MariaDB)]

    Client -->|User Interaction| Slint
    Slint -->|HTTP Requests| Actix
    Actix -->|SQL Queries| SQLx
    SQLx -->|TCP| DB
```

### 2.3 Slint Frontend - Component Architecture

The frontend is built using **Slint**, a declarative GUI toolkit.

```slint
// ui/app-window.slint
import { Button, VerticalBox, ScrollView } from "std-widgets.slint";
import { SideBar } from "side_bar.slint";
import { TitlesPage } from "pages/titles_page.slint";
// ... other imports

export component AppWindow inherits Window {
    title: @tr("Personal Library");
    min-width: 1024px;
    min-height: 768px;

    // Global Properties
    in-out property <[TitleData]> titles: [];
    in-out property <[AuthorData]> authors: [];
    // ...

    // Callbacks to Rust Backend
    callback load-titles();
    callback create-title(string, string, ...);
    
    HorizontalLayout {
        // Navigation Sidebar
        side-bar := SideBar {
            model: [
                @tr("Titles"), 
                @tr("Locations"), 
                // ...
            ];
            
            changed current-item => {
                // Route navigation logic
            }
        }

        // Main Content Area
        if(side-bar.current-item == 0): TitlesPage {
            titles: root.titles;
            load-titles => { root.load-titles(); }
        }
        if(side-bar.current-item == 1): LocationsPage { /* ... */ }
        // ...
    }
}
```

**Key Concepts:**

- **.slint files**: Define the UI structure and styling declaratively.
- **AppWindow**: Root component that manages global state and routing.
- **Callbacks**: Mechanism for UI to trigger Rust backend logic (e.g., `load-titles`).
- **Properties**: Data flow from Rust to UI (e.g., `titles` list).
- **WASM support**: Compiles to WebAssembly for running in browser, communicating via `reqwest-wasm`.

## 3. Data Model

### 3.1 Main entities - Title/Volume Hierarchy

```rust
// models/title.rs
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Title {
    pub id: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub isbn: Option<String>,
    pub publisher: Option<String>,
    pub publication_year: Option<i32>,
    pub pages: Option<i32>,
    pub language: String,
    pub dewey_code: Option<String>,
    pub dewey_category: Option<String>,
    pub genre: Option<String>,
    pub summary: Option<String>,
    pub cover_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// models/volume.rs
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Volume {
    pub id: Uuid,
    pub title_id: Uuid,                    // Foreign key to Title
    pub copy_number: i32,                  // Copy number (1, 2, 3...)
    pub barcode: String,                   // Unique barcode VOL-000001
    pub condition: VolumeCondition,        // Physical condition
    pub location: Option<String>,          // Physical location
    pub loan_status: LoanStatus,           // Loan status
    pub individual_notes: Option<String>,  // Notes specific to this copy
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "volume_condition", rename_all = "lowercase")]
pub enum VolumeCondition {
    Excellent,
    Good,
    Fair,
    Poor,
    Damaged,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "loan_status", rename_all = "lowercase")]
pub enum LoanStatus {
    Available,
    Loaned,
    Overdue,
    Lost,
    Reserved,
}

// models/title_with_volumes.rs (for queries)
#[derive(Debug, Serialize, Deserialize)]
pub struct TitleWithVolumes {
    pub title: Title,
    pub volumes: Vec<Volume>,
    pub authors: Vec<Author>,
    pub total_volumes: i32,
    pub available_volumes: i32,
}
```

### 3.2 Relations and Additional Entities

```rust
// models/author.rs
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Author {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: String,
    pub pseudonym: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub death_date: Option<NaiveDate>,
    pub nationality: Option<String>,
    pub biography: Option<String>,
    pub photo_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// models/title_author.rs (Many-to-many relationship)
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TitleAuthor {
    pub title_id: Uuid,
    pub author_id: Uuid,
    pub role: AuthorRole,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "author_role", rename_all = "lowercase")]
pub enum AuthorRole {
    MainAuthor,
    CoAuthor,
    Illustrator,
    Translator,
    Editor,
}

// models/series.rs
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Series {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: SeriesStatus,
    pub total_titles: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// models/loan.rs
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Loan {
    pub id: Uuid,
    pub title_id: Uuid,                    // Reference to title (not specific volume)
    pub volume_id: Uuid,                   // Specific volume selected by system
    pub borrower_id: Uuid,
    pub loan_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub return_date: Option<DateTime<Utc>>,
    pub status: LoanStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// models/borrower.rs (Simplified for personal use)
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Borrower {
    pub id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// models/wishlist.rs (Simple wishlist instead of complex reservations)
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct WishlistItem {
    pub id: Uuid,
    pub title_id: Uuid,
    pub borrower_id: Uuid,
    pub added_date: DateTime<Utc>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// models/duplicate.rs (For duplicate detection)
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DuplicateCandidate {
    pub id: Uuid,
    pub title_id_1: Uuid,
    pub title_id_2: Uuid,
    pub confidence_score: f32,
    pub detection_type: DuplicateType,
    pub status: DuplicateStatus,
    pub created_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "duplicate_type", rename_all = "lowercase")]
pub enum DuplicateType {
    IdenticalIsbn,
    TitleAuthorMatch,
    FuzzyMatch,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "duplicate_status", rename_all = "lowercase")]
pub enum DuplicateStatus {
    Pending,
    Confirmed,
    Ignored,
    Merged,
}

// models/title_type.rs (For loan duration rules)
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "title_type", rename_all = "lowercase")]
pub enum TitleType {
    Fiction,
    NonFiction,
    Reference,
    Magazine,
    RareBook,
    Custom,
}

impl TitleType {
    pub fn default_loan_days(&self) -> i32 {
        match self {
            TitleType::Fiction => 21,
            TitleType::NonFiction => 14,
            TitleType::Reference => 7,
            TitleType::Magazine => 3,
            TitleType::RareBook => 7,
            TitleType::Custom => 14,
        }
    }
}
```

## 4. REST API

### 4.1 Endpoint structure

### 4.1 Implementation Pattern

The backend uses **Actix Web** with the following pattern:

```rust
pub async fn handler_name(
    data: web::Data<AppState>,      // Database pool
    path: web::Path<Uuid>,          // Path parameters
    json: web::Json<MyPayload>,     // JSON body
    query: web::Query<MyParams>,    // Query parameters
) -> impl Responder {
    // Implementation
}
```

### 4.2 Basic API Routes for Personal Use

### 4.2 Actix Handler Implementation

```rust
// handlers/titles.rs
use actix_web::{web, HttpResponse, Responder};
use sqlx::MySqlPool;

pub async fn list_titles(data: web::Data<AppState>) -> impl Responder {
    let query = "SELECT * FROM titles";
    // ... Direct SQLx query implementation
}

pub async fn create_title(
    data: web::Data<AppState>,
    Json(payload): Json<CreateTitleRequest>,
) -> impl Responder {
    // ... Logic to create title and optional volume
}
```

### 4.3 Simplified API Endpoints (Personal Use)

```
# Essential Title Management
GET    /api/v1/titles                    - List titles with basic information
POST   /api/v1/titles                    - Create a title
GET    /api/v1/titles/{id}               - Title details with volumes
PUT    /api/v1/titles/{id}               - Update title
DELETE /api/v1/titles/{id}               - Delete title
GET    /api/v1/titles/wishlist           - Titles with 0 volumes

# Volume Management
POST   /api/v1/titles/{id}/volumes       - Add volume to title
PUT    /api/v1/volumes/{id}              - Update volume
DELETE /api/v1/volumes/{id}              - Delete volume

# Basic Loan Operations
POST   /api/v1/loans                     - Create loan
PUT    /api/v1/loans/{id}/return         - Return volume
GET    /api/v1/loans/active              - Active loans

# Scanner Support (Dual Barcode)
GET    /api/v1/scan/volume/{barcode}     - Find volume by Code 128 barcode
GET    /api/v1/scan/isbn/{isbn}          - Find title by EAN-13 ISBN barcode
POST   /api/v1/scan/loan                 - Loan via volume barcode scan
POST   /api/v1/scan/return               - Return via volume barcode scan
POST   /api/v1/scan/add-title            - Add title via ISBN barcode scan

# Simple Wishlist (instead of complex reservations)
GET    /api/v1/wishlist/{borrower_id}    - Get borrower's wishlist
POST   /api/v1/wishlist                  - Add title to wishlist
DELETE /api/v1/wishlist/{id}             - Remove from wishlist

# Duplicate Management
GET    /api/v1/duplicates                - List potential duplicates
POST   /api/v1/duplicates/merge          - Merge two titles
POST   /api/v1/duplicates/ignore         - Mark as non-duplicate
GET    /api/v1/duplicates/suggestions/{title-id} - Get suggestions for title

# Basic Search
GET    /api/v1/search/titles             - Search titles
GET    /api/v1/search/volumes            - Search volumes

# Simple Reports
GET    /api/v1/reports/basic             - Basic collection statistics
GET    /api/v1/reports/loans             - Simple loan reports
```

## 5. Backend Architecture & Data Flow

### 5.1 Architecture Pattern: Handler-Centric

The application follows a pragmatic **Handler-Centric** architecture, suitable for its scope as a personal library manager.

- **Actix Web Handlers**: Serve as the entry points for API requests. They handle:
  - Request extraction (JSON body, Path parameters, Query strings)
  - Business logic validation
  - Direct database interactions via `sqlx`
  - Response formatting (JSON)
- **AppState**: Holds shared resources, primarily the `sqlx::MySqlPool` for database connections.
- **Models**: Rust structs that map to database tables (`sqlx::FromRow`) and API responses (`serde::Serialize`).

This approach minimizes boilerplate by avoiding unnecessary Service/Repository abstraction layers for simple CRUD operations.

### 5.2 Database Access

Database interactions are performed directly within handlers using **SQLx**. This provides:

- **Compile-time verification**: SQL queries are checked against the database schema at compile time.
- **Async/Await**: Non-blocking database I/O.
- **Type Safety**: Automatic mapping from SQL types to Rust types.

```rust
// Example: Fetching a title by ID
pub async fn get_title(data: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    let query = "SELECT * FROM titles WHERE id = ?";
    match sqlx::query_as::<_, Title>(query)
        .bind(id.into_inner())
        .fetch_optional(&data.db_pool)
        .await 
    {
        Ok(Some(title)) => HttpResponse::Ok().json(title),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("Database error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
```

### 6.3 Multi-DBMS migrations

```rust
// migrations/migration_runner.rs
pub struct MigrationRunner {
    database_type: String,
}

impl MigrationRunner {
    pub async fn run_migrations(&self, pool: &DatabaseType) -> Result<(), MigrationError> {
        let migration_files = match self.database_type.as_str() {
            "postgresql" => self.load_postgres_migrations(),
            "mysql" | "mariadb" => self.load_mysql_migrations(),
            _ => return Err(MigrationError::UnsupportedDatabase),
        };
        
        for migration in migration_files {
            self.execute_migration(pool, &migration).await?;
        }
        
        Ok(())
    }
}
```

```sql
-- migrations/postgresql/001_initial_title_volume.sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Enums
CREATE TYPE volume_condition AS ENUM ('excellent', 'good', 'fair', 'poor', 'damaged');
CREATE TYPE loan_status AS ENUM ('available', 'loaned', 'overdue', 'lost', 'reserved');
CREATE TYPE author_role AS ENUM ('main_author', 'co_author', 'illustrator', 'translator', 'editor');
CREATE TYPE reservation_status AS ENUM ('active', 'fulfilled', 'expired', 'cancelled');
CREATE TYPE location_type AS ENUM ('building', 'room', 'shelf', 'section');

-- Authors table
CREATE TABLE authors (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    first_name VARCHAR(255),
    last_name VARCHAR(255) NOT NULL,
    pseudonym VARCHAR(255),
    birth_date DATE,
    death_date DATE,
    nationality VARCHAR(100),
    biography TEXT,
    photo_url VARCHAR(500),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Titles table (main metadata)
CREATE TABLE titles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(500) NOT NULL,
    subtitle VARCHAR(500),
    isbn VARCHAR(20),
    publisher VARCHAR(255),
    publication_year INTEGER,
    pages INTEGER,
    language VARCHAR(10) DEFAULT 'en',
    dewey_code VARCHAR(10),
    dewey_category VARCHAR(255),
    genre VARCHAR(100),
    summary TEXT,
    cover_url VARCHAR(500),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Locations table (hierarchical)
CREATE TABLE locations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    parent_id UUID REFERENCES locations(id) ON DELETE CASCADE,
    location_type location_type NOT NULL,
    capacity INTEGER,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Volumes table (physical copies)
CREATE TABLE volumes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title_id UUID NOT NULL REFERENCES titles(id) ON DELETE CASCADE,
    copy_number INTEGER NOT NULL,
    barcode VARCHAR(50) UNIQUE NOT NULL,
    condition volume_condition DEFAULT 'good',
    location_id UUID REFERENCES locations(id),
    loan_status loan_status DEFAULT 'available',
    individual_notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(title_id, copy_number)
);

-- Title-Author relationship (many-to-many)
CREATE TABLE title_authors (
    title_id UUID NOT NULL REFERENCES titles(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES authors(id) ON DELETE CASCADE,
    role author_role DEFAULT 'main_author',
    PRIMARY KEY (title_id, author_id, role)
);

-- Series table
CREATE TABLE series (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    total_titles INTEGER,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Title-Series relationship
CREATE TABLE title_series (
    title_id UUID NOT NULL REFERENCES titles(id) ON DELETE CASCADE,
    series_id UUID NOT NULL REFERENCES series(id) ON DELETE CASCADE,
    series_number INTEGER,
    PRIMARY KEY (title_id, series_id)
);

-- Borrowers table
CREATE TABLE borrowers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    phone VARCHAR(50),
    address TEXT,
    max_loans INTEGER DEFAULT 5,
    is_suspended BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Simplified borrowers table (no complex restrictions)
CREATE TABLE borrowers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    phone VARCHAR(50),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Simple wishlist instead of complex reservations
CREATE TABLE wishlist_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title_id UUID NOT NULL REFERENCES titles(id) ON DELETE CASCADE,
    borrower_id UUID NOT NULL REFERENCES borrowers(id) ON DELETE CASCADE,
    added_date TIMESTAMPTZ DEFAULT NOW(),
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(title_id, borrower_id)
);

-- Duplicate detection table
CREATE TABLE duplicate_candidates (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title_id_1 UUID NOT NULL REFERENCES titles(id) ON DELETE CASCADE,
    title_id_2 UUID NOT NULL REFERENCES titles(id) ON DELETE CASCADE,
    confidence_score REAL NOT NULL,
    detection_type duplicate_type NOT NULL,
    status duplicate_status DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    reviewed_at TIMESTAMPTZ,
    
    UNIQUE(title_id_1, title_id_2)
);

-- Loans table (simplified)
CREATE TABLE loans (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title_id UUID NOT NULL REFERENCES titles(id),
    volume_id UUID NOT NULL REFERENCES volumes(id),
    borrower_id UUID NOT NULL REFERENCES borrowers(id),
    loan_date TIMESTAMPTZ DEFAULT NOW(),
    due_date TIMESTAMPTZ NOT NULL,
    return_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_volumes_title_id ON volumes(title_id);
CREATE INDEX idx_volumes_barcode ON volumes(barcode);
CREATE INDEX idx_volumes_loan_status ON volumes(loan_status);
CREATE INDEX idx_titles_isbn ON titles(isbn);
CREATE INDEX idx_titles_title ON titles USING gin(to_tsvector('english', title));
CREATE INDEX idx_titles_title_type ON titles(title_type);
CREATE INDEX idx_title_authors_title_id ON title_authors(title_id);
CREATE INDEX idx_title_authors_author_id ON title_authors(author_id);
CREATE INDEX idx_loans_volume_id ON loans(volume_id);
CREATE INDEX idx_loans_borrower_id ON loans(borrower_id);
CREATE INDEX idx_loans_due_date ON loans(due_date);
CREATE INDEX idx_wishlist_items_title_id ON wishlist_items(title_id);
CREATE INDEX idx_wishlist_items_borrower_id ON wishlist_items(borrower_id);
CREATE INDEX idx_duplicate_candidates_status ON duplicate_candidates(status);
CREATE INDEX idx_duplicate_candidates_confidence ON duplicate_candidates(confidence_score);
```

```sql
-- migrations/mysql/001_initial.sql
CREATE TABLE authors (
    id CHAR(36) PRIMARY KEY,
    first_name VARCHAR(255),
    last_name VARCHAR(255) NOT NULL,
    pseudonym VARCHAR(255),
    birth_date DATE,
    death_date DATE,
    nationality VARCHAR(100),
    biography TEXT,
    photo_url VARCHAR(500),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE volumes (
    id CHAR(36) PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    subtitle VARCHAR(500),
    isbn VARCHAR(20),
    publisher VARCHAR(255),
    publication_year INTEGER,
    pages INTEGER,
    language VARCHAR(10) DEFAULT 'en',
    dewey_code VARCHAR(10),
    dewey_category VARCHAR(255),
    genre VARCHAR(100),
    summary TEXT,
    cover_url VARCHAR(500),
    location VARCHAR(255),
    condition ENUM('excellent', 'good', 'fair', 'poor', 'lost') DEFAULT 'good',
    barcode VARCHAR(50) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE INDEX idx_volumes_barcode ON volumes(barcode);
CREATE INDEX idx_volumes_isbn ON volumes(isbn);
CREATE FULLTEXT INDEX idx_volumes_title ON volumes(title);
```

## 7. Authentication and Security

### 7.1 JWT and middleware

```rust
// middleware/auth.rs
pub async fn auth_middleware(
    State(app_state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_token_from_header(&req)?;
    let claims = validate_jwt_token(&token, &app_state.jwt_secret)?;
    
    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: UserRole,
    pub exp: usize,
}
```

### 7.2 Data validation

```rust
// utils/validation.rs
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateVolumeRequest {
    #[validate(length(min = 1, max = 500))]
    pub title: String,
    
    #[validate(custom = "validate_isbn")]
    pub isbn: Option<String>,
    
    #[validate(range(min = 1000, max = 2100))]
    pub publication_year: Option<i32>,
}

fn validate_isbn(isbn: &str) -> Result<(), ValidationError> {
    // ISBN-10 or ISBN-13 validation
}
```

## 8. Internationalization

### 8.1 Backend - i18n structure

```rust
// services/i18n_service.rs
pub struct I18nService {
    translations: HashMap<String, HashMap<String, String>>,
}

impl I18nService {
    pub fn translate(&self, key: &str, locale: &str) -> String {
        self.translations
            .get(locale)
            .and_then(|t| t.get(key))
            .unwrap_or(key)
            .to_string()
    }
}
```

### 8.2 Backend - Locale management

```rust
// middleware/locale.rs
pub async fn locale_middleware(
    mut req: Request,
    next: Next,
) -> Response {
    let locale = extract_locale_from_header(&req)
        .unwrap_or_else(|| "en".to_string());
    
    req.extensions_mut().insert(Locale(locale));
    next.run(req).await
}
```

### 8.3 Frontend - API Client service

```rust
// frontend/src/services/api_client.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn get_volumes(&self, params: VolumeQueryParams) -> Result<PaginatedResponse<Volume>, ApiError> {
        let url = format!("{}/api/v1/volumes", self.base_url);
        let response = self.client
            .get(&url)
            .query(&params)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(ApiError::HttpError(response.status()))
        }
    }

    pub async fn scan_barcode(&self, barcode: &str) -> Result<Option<Volume>, ApiError> {
        let url = format!("{}/api/v1/scan/{}", self.base_url, barcode);
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        match response.status() {
            reqwest::StatusCode::OK => Ok(Some(response.json().await?)),
            reqwest::StatusCode::NOT_FOUND => Ok(None),
            status => Err(ApiError::HttpError(status)),
        }
    }
}
```

### 8.4 Frontend - Simplified Global State for Personal Use

```rust
// frontend/src/services/state.rs
use leptos::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct AppState {
    pub titles: RwSignal<Vec<TitleWithVolumes>>,
    pub current_title: RwSignal<Option<TitleWithVolumes>>,
    pub current_user: RwSignal<Option<User>>,
    pub loading: RwSignal<bool>,
    pub notifications: RwSignal<Vec<Notification>>,
    pub wishlist: RwSignal<Vec<WishlistItem>>,
    pub duplicates: RwSignal<Vec<DuplicateCandidate>>,
    pub i18n: RwSignal<HashMap<String, String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            titles: create_rw_signal(Vec::new()),
            current_title: create_rw_signal(None),
            current_user: create_rw_signal(None),
            loading: create_rw_signal(false),
            notifications: create_rw_signal(Vec::new()),
            wishlist: create_rw_signal(Vec::new()),
            duplicates: create_rw_signal(Vec::new()),
            i18n: create_rw_signal(HashMap::new()),
        }
    }

    pub fn add_notification(&self, message: String, level: NotificationLevel) {
        let notification = Notification {
            id: uuid::Uuid::new_v4(),
            message,
            level,
            timestamp: chrono::Utc::now(),
        };
        
        self.notifications.update(|notifications| {
            notifications.push(notification);
        });
    }

    pub fn update_title(&self, updated_title: TitleWithVolumes) {
        self.titles.update(|titles| {
            if let Some(pos) = titles.iter().position(|t| t.title.id == updated_title.title.id) {
                titles[pos] = updated_title;
            }
        });
    }

    pub fn add_volume_to_title(&self, title_id: Uuid, new_volume: Volume) {
        self.titles.update(|titles| {
            if let Some(title_with_volumes) = titles.iter_mut().find(|t| t.title.id == title_id) {
                title_with_volumes.volumes.push(new_volume);
                title_with_volumes.total_volumes += 1;
                if new_volume.loan_status == LoanStatus::Available {
                    title_with_volumes.available_volumes += 1;
                }
            }
        });
    }

    pub fn add_to_wishlist(&self, item: WishlistItem) {
        self.wishlist.update(|wishlist| {
            wishlist.push(item);
        });
    }

    pub fn remove_from_wishlist(&self, item_id: Uuid) {
        self.wishlist.update(|wishlist| {
            wishlist.retain(|item| item.id != item_id);
        });
    }

    pub fn add_duplicate_candidate(&self, candidate: DuplicateCandidate) {
        self.duplicates.update(|duplicates| {
            duplicates.push(candidate);
        });
    }

    pub fn remove_duplicate_candidate(&self, candidate_id: Uuid) {
        self.duplicates.update(|duplicates| {
            duplicates.retain(|dup| dup.id != candidate_id);
        });
    }
}

// Global context
pub fn provide_app_state() {
    provide_context(AppState::new());
}

pub fn use_app_state() -> AppState {
    expect_context::<AppState>()
}
```

## 9. Configuration and Deployment

### 9.1 Configuration

```rust
// config/mod.rs
#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub external_apis: ExternalApisConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
}

// config/database.rs
#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub database_type: String, // "postgresql", "mysql", "mariadb"
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        match self.database_type.as_str() {
            "postgresql" => format!(
                "postgresql://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database_name
            ),
            "mysql" | "mariadb" => format!(
                "mysql://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database_name
            ),
            _ => panic!("Unsupported database type: {}", self.database_type),
        }
    }
}
```

### 9.2 Docker

#### Backend Dockerfile

```dockerfile
# backend/Dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/library-manager /usr/local/bin/
EXPOSE 8000
CMD ["library-manager"]
```

#### Frontend Dockerfile

```dockerfile
# frontend/Dockerfile
FROM rust:1.75 as builder

# Install Trunk and wasm-pack
RUN cargo install trunk wasm-pack
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY index.html Trunk.toml ./

# Build WASM frontend
RUN trunk build --release

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

### 9.3 Multi-DBMS Docker Compose

```yaml
# docker-compose.yml
version: '3.8'
services:
  backend:
    build: ./backend
    ports:
      - "8000:8000"
    environment:
      - DATABASE_TYPE=${DATABASE_TYPE:-postgresql}
      - DATABASE_URL=${DATABASE_URL}
      - REDIS_URL=redis://redis:6379
    depends_on:
      - db

  frontend:
    build: ./frontend
    ports:
      - "80:80"
    depends_on:
      - backend

  # PostgreSQL (default)
  db-postgres:
    image: postgres:15
    profiles: ["postgres"]
    environment:
      POSTGRES_DB: library
      POSTGRES_USER: user
      POSTGRES_PASSWORD: pass
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

  # MySQL
  db-mysql:
    image: mysql:8.0
    profiles: ["mysql"]
    environment:
      MYSQL_DATABASE: library
      MYSQL_USER: user
      MYSQL_PASSWORD: pass
      MYSQL_ROOT_PASSWORD: rootpass
    volumes:
      - mysql_data:/var/lib/mysql
    ports:
      - "3306:3306"

  # MariaDB
  db-mariadb:
    image: mariadb:10.11
    profiles: ["mariadb"]
    environment:
      MARIADB_DATABASE: library
      MARIADB_USER: user
      MARIADB_PASSWORD: pass
      MARIADB_ROOT_PASSWORD: rootpass
    volumes:
      - mariadb_data:/var/lib/mysql
    ports:
      - "3306:3306"

  redis:
    image: redis:7-alpine
    
volumes:
  postgres_data:
  mysql_data:
  mariadb_data:
```

### 9.4 Cargo.toml configuration

#### Backend Cargo.toml

```toml
[package]
name = "library-manager-backend"
version = "0.1.0"
edition = "2021"

[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "uuid", "chrono"] }

# Conditional features for DBMS
[features]
default = ["postgresql"]
postgresql = ["sqlx/postgres"]
mysql = ["sqlx/mysql"]
mariadb = ["mysql"] # MariaDB uses MySQL driver

[dependencies.sqlx]
version = "0.7"
features = ["runtime-tokio-rustls", "uuid", "chrono"]
optional-features = ["postgres", "mysql"]
```

#### Frontend Cargo.toml

```toml
[package]
name = "library-manager-frontend"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
leptos = { version = "0.5", features = ["csr"] }
leptos_router = { version = "0.5", features = ["csr"] }
leptos_meta = { version = "0.5", features = ["csr"] }
leptos-i18n = { version = "0.3", features = ["csr"] }
console_error_panic_hook = "0.1"
console_log = "1.0"
log = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.11", features = ["json"] }
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = "0.3"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde", "wasmbind"] }
gloo-timers = { version = "0.3", features = ["futures"] }
gloo-storage = "0.3"

[dependencies.web-sys]
version = "0.3"
features = [
  "console",
  "HtmlInputElement",
  "HtmlSelectElement",
  "HtmlTextAreaElement",
  "KeyboardEvent",
  "Event",
  "EventTarget",
  "File",
  "FileList",
  "FormData",
  "Headers",
  "Request",
  "RequestInit",
  "Response",
  "Window",
  "Document",
  "Element",
  "HtmlElement",
  "Storage",
  "Location",
]
```

#### Frontend Trunk.toml

```toml
[build]
target = "index.html"
dist = "dist"

[watch]
watch = ["src", "Cargo.toml"]
ignore = ["dist"]

[serve]
address = "127.0.0.1"
port = 3000
open = false

[[hooks]]
stage = "pre_build"
command = "tailwindcss"
command_arguments = ["-i", "./input.css", "-o", "./style.css", "--watch"]
```

### 9.5 Deployment scripts

```bash
# scripts/deploy-postgres.sh
export DATABASE_TYPE=postgresql
export DATABASE_URL=postgresql://user:pass@localhost:5432/library
docker-compose --profile postgres up -d

# scripts/deploy-mysql.sh
export DATABASE_TYPE=mysql
export DATABASE_URL=mysql://user:pass@localhost:3306/library
docker-compose --profile mysql up -d

# scripts/deploy-mariadb.sh
export DATABASE_TYPE=mariadb
export DATABASE_URL=mysql://user:pass@localhost:3306/library
docker-compose --profile mariadb up -d
```

## 10. Testing and Quality

### 10.1 Test structure

Integration tests are located in `backend/tests` and use the standard Actix Web testing utilities.

```rust
// tests/health_check.rs
#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
```

### 10.2 Quality tools

- **Unit tests**: `cargo test` in both frontend and backend
- **Integration tests**: `cargo test` in backend (uses `sqlx` test database)
- **Linting**: `clippy` for Rust best practices
- **Formatting**: `rustfmt` for consistent code style
- **Audit**: `cargo audit` to check for security vulnerabilities in dependencies

## 11. Performance and Monitoring

### 11.1 Optimizations

- **Connection pooling**: `sqlx` connection pool for efficient database access
- **Async I/O**: Tokio runtime for non-blocking operations in connection handling
- **Native UI**: Slint provides a lightweight, compiled native UI (no heavy browser DOM)
- **Database indexes**: Optimized for common search queries
- **Resource Management**: Minimal memory footprint compared to Electron-based apps

### 11.2 Monitoring

Basic logging is implemented using `tracing` and `tracing-subscriber`.

```rust
// backend/src/main.rs (setup)
let subscriber = get_subscriber("rbibli", "info", std::io::stdout);
init_subscriber(subscriber);
```

### 11.3 Scanners and Hardware Support

The input system relies on standard HID (Human Interface Device) support, making it compatible with most USB and Bluetooth barcode scanners.

- **Frontend Handling**: Slint's `FocusScope` and `TextInput` components capture scanner input (typically followed by an Enter keycode).
- **Dual Support**: The system differentiates between:
  - **ISBN (EAN-13)**: For cataloging and adding new items.
  - **Volume Barcodes (Code 128 - VOL-XXXXXX)**: For loan and return operations.

This architecture ensures high performance and reliability for a personal library management system, leveraging the safety and speed of Rust across the entire stack.
