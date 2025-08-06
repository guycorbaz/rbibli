# Technical Architecture - Personal Library Manager

## Overview
Web application developed in Rust with a modular, performant and secure architecture for managing a personal library.

## 1. Technical Stack

### 1.1 Backend (Rust)
- **Web framework**: Axum (async, performant, type-safe)
- **Database**: PostgreSQL, MySQL or MariaDB with SQLx (async, compile-time checked queries)
- **ORM/Query Builder**: SQLx + multi-DBMS migrations
- **Authentication**: JWT with jsonwebtoken
- **Validation**: validator crate
- **Serialization**: serde (JSON/XML)
- **Logging**: tracing + tracing-subscriber
- **Configuration**: config crate + environment variables

### 1.2 Frontend (Rust with Leptos)
- **Web framework**: Leptos (reactive Rust framework for SPA)
- **Compilation**: WebAssembly (WASM) for optimal performance
- **CSS Framework**: Tailwind CSS
- **Global state**: Leptos Context API and reactive signals
- **HTTP Client**: reqwest-wasm
- **UI Components**: Custom Leptos components
- **Internationalization**: leptos-i18n
- **Build tool**: Trunk for WASM bundling
- **Reactivity**: Fine-grained signals system

### 1.3 Infrastructure
- **Containerization**: Docker + Docker Compose
- **Reverse Proxy**: Nginx
- **Database**: PostgreSQL 15+, MySQL 8.0+ or MariaDB 10.11+
- **Cache**: Redis (sessions, API cache)
- **File storage**: Local filesystem or S3-compatible

## 2. General Architecture

### 2.1 Project structure
```
library-manager/
├── backend/
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── config/
│   │   ├── models/
│   │   ├── handlers/
│   │   ├── services/
│   │   ├── repositories/
│   │   ├── middleware/
│   │   ├── utils/
│   │   └── migrations/
│   ├── Cargo.toml
│   └── Dockerfile
├── frontend/
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── app.rs
│   │   ├── components/
│   │   │   ├── mod.rs
│   │   │   ├── volume/
│   │   │   ├── author/
│   │   │   ├── loan/
│   │   │   └── common/
│   │   ├── pages/
│   │   │   ├── mod.rs
│   │   │   ├── dashboard.rs
│   │   │   ├── volumes.rs
│   │   │   └── loans.rs
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   ├── api_client.rs
│   │   │   └── state.rs
│   │   ├── utils/
│   │   └── i18n/
│   ├── Cargo.toml
│   ├── Trunk.toml
│   ├── index.html
│   └── Dockerfile
├── docker-compose.yml
└── documentation/
```

### 2.2 Layered architecture
```
┌─────────────────────────────────────┐
│      Frontend Rust (WASM)           │
├─────────────────────────────────────┤
│              Nginx                  │
├─────────────────────────────────────┤
│          REST API (Axum)            │
├─────────────────────────────────────┤
│        Business Services            │
├─────────────────────────────────────┤
│         Repositories                │
├─────────────────────────────────────┤
│      PostgreSQL/MySQL + Redis       │
└─────────────────────────────────────┘
```

### 2.3 Leptos Frontend - Reactive architecture
```rust
// src/app.rs
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Global application state
    provide_context(create_rw_signal(AppState::new()));
    
    view! {
        <Router>
            <nav class="bg-blue-600 text-white p-4">
                <div class="container mx-auto flex justify-between">
                    <h1 class="text-xl font-bold">"Library Manager"</h1>
                    <div class="space-x-4">
                        <A href="/" class="hover:underline">"Home"</A>
                        <A href="/volumes" class="hover:underline">"Volumes"</A>
                        <A href="/loans" class="hover:underline">"Loans"</A>
                        <A href="/scan" class="hover:underline">"Scanner"</A>
                    </div>
                </div>
            </nav>
            
            <main class="container mx-auto p-4">
                <Routes>
                    <Route path="/" view=Dashboard/>
                    <Route path="/volumes" view=VolumesPage/>
                    <Route path="/volumes/new" view=VolumeForm/>
                    <Route path="/volumes/:id" view=VolumeDetail/>
                    <Route path="/loans" view=LoansPage/>
                    <Route path="/scan" view=ScanPage/>
                </Routes>
            </main>
            
            <NotificationCenter />
        </Router>
    }
}

// src/components/volume/volume_card.rs
#[component]
pub fn VolumeCard(volume: Volume) -> impl IntoView {
    let is_available = move || volume.loan_status == LoanStatus::Available;
    
    view! {
        <div class="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow">
            <div class="flex items-start space-x-4">
                <img 
                    src={volume.cover_url.unwrap_or_else(|| "/default-cover.jpg".to_string())}
                    alt={format!("Cover of {}", volume.title)}
                    class="w-16 h-24 object-cover rounded"
                />
                <div class="flex-1">
                    <h3 class="text-lg font-semibold text-gray-900">{volume.title}</h3>
                    {volume.subtitle.map(|subtitle| view! {
                        <p class="text-sm text-gray-600">{subtitle}</p>
                    })}
                    <p class="text-sm text-gray-500 mt-1">
                        "Barcode: " <span class="font-mono">{volume.barcode}</span>
                    </p>
                    <div class="flex items-center mt-2 space-x-2">
                        <span class={format!("px-2 py-1 rounded-full text-xs font-medium {}", 
                            condition_color(&volume.condition))}>
                            {condition_label(&volume.condition)}
                        </span>
                        {volume.dewey_code.map(|code| view! {
                            <span class="px-2 py-1 bg-blue-100 text-blue-800 rounded-full text-xs">
                                {code}
                            </span>
                        })}
                        <Show when=move || !is_available()>
                            <span class="px-2 py-1 bg-red-100 text-red-800 rounded-full text-xs">
                                "On Loan"
                            </span>
                        </Show>
                    </div>
                </div>
            </div>
        </div>
    }
}

// src/components/volume/volumes_list.rs
#[component]
pub fn VolumesList() -> impl IntoView {
    let volumes = create_resource(|| (), |_| async { 
        api_client().get_volumes().await 
    });

    view! {
        <div class="space-y-4">
            <Suspense fallback=move || view! { 
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    {(0..6).map(|_| view! {
                        <div class="animate-pulse bg-gray-200 h-48 rounded-lg"></div>
                    }).collect_view()}
                </div>
            }>
                {move || volumes.get().map(|vols| match vols {
                    Ok(volumes) => view! {
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                            <For
                                each=move || volumes.clone()
                                key=|vol| vol.id
                                children=move |vol| view! { <VolumeCard volume=vol/> }
                            />
                        </div>
                    }.into_view(),
                    Err(_) => view! { 
                        <div class="text-center py-8">
                            <p class="text-red-600">"Error loading volumes"</p>
                            <button 
                                class="mt-2 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
                                on:click=move |_| volumes.refetch()
                            >
                                "Retry"
                            </button>
                        </div>
                    }.into_view()
                })}
            </Suspense>
        </div>
    }
}
```

## 3. Data Model

### 3.1 Main entities
```rust
// models/volume.rs
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Volume {
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
    pub location: Option<String>,
    pub condition: VolumeCondition,
    pub barcode: String,
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
    Lost,
}
```

### 3.2 Relations
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
}

// models/series.rs
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Series {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: SeriesStatus,
    pub total_volumes: Option<i32>,
}

// models/loan.rs
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Loan {
    pub id: Uuid,
    pub volume_id: Uuid,
    pub borrower_id: Uuid,
    pub loan_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub return_date: Option<DateTime<Utc>>,
    pub status: LoanStatus,
}
```

## 4. REST API

### 4.1 Endpoint structure
```rust
// handlers/volumes.rs
pub async fn create_volume(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateVolumeRequest>,
) -> Result<Json<Volume>, AppError> {
    // Implementation
}

pub async fn get_volumes(
    State(app_state): State<AppState>,
    Query(params): Query<VolumeQueryParams>,
) -> Result<Json<PaginatedResponse<Volume>>, AppError> {
    // Implementation
}
```

### 4.2 Main routes
```
GET    /api/v1/volumes              - Paginated list of volumes
POST   /api/v1/volumes              - Create a volume
GET    /api/v1/volumes/{id}         - Volume details
PUT    /api/v1/volumes/{id}         - Update a volume
DELETE /api/v1/volumes/{id}         - Delete a volume

GET    /api/v1/authors              - List of authors
POST   /api/v1/authors              - Create an author
GET    /api/v1/authors/{id}/volumes - Volumes by an author

GET    /api/v1/series               - List of series
POST   /api/v1/series               - Create a series
GET    /api/v1/series/{id}/volumes  - Volumes in a series

POST   /api/v1/loans                - Create a loan
PUT    /api/v1/loans/{id}/return    - Return a volume
GET    /api/v1/loans/overdue        - Overdue loans

GET    /api/v1/scan/{barcode}       - Search by barcode
POST   /api/v1/scan/loan            - Loan via scan
POST   /api/v1/scan/return          - Return via scan

GET    /api/v1/stats/dashboard      - General statistics
GET    /api/v1/reports/inventory    - Inventory report
```

## 5. Services and Business Logic

### 5.1 Service structure
```rust
// services/volume_service.rs
pub struct VolumeService {
    volume_repo: Arc<dyn VolumeRepository>,
    author_repo: Arc<dyn AuthorRepository>,
    barcode_generator: Arc<dyn BarcodeGenerator>,
}

impl VolumeService {
    pub async fn create_volume(
        &self,
        request: CreateVolumeRequest,
    ) -> Result<Volume, ServiceError> {
        // Validation
        // Generate unique barcode
        // Fetch external metadata (ISBN)
        // Save to database
    }

    pub async fn search_volumes(
        &self,
        criteria: SearchCriteria,
    ) -> Result<PaginatedResponse<Volume>, ServiceError> {
        // Advanced search logic
    }
}
```

### 5.2 Barcode management
```rust
// services/barcode_service.rs
pub trait BarcodeGenerator: Send + Sync {
    async fn generate_unique_barcode(&self) -> Result<String, BarcodeError>;
    fn validate_barcode(&self, barcode: &str) -> bool;
}

pub struct SequentialBarcodeGenerator {
    repo: Arc<dyn BarcodeRepository>,
}

impl BarcodeGenerator for SequentialBarcodeGenerator {
    async fn generate_unique_barcode(&self) -> Result<String, BarcodeError> {
        let next_id = self.repo.get_next_sequence().await?;
        Ok(format!("LIB-{:06}", next_id))
    }
}
```

## 6. Data Management

### 6.1 Multi-DBMS abstraction
```rust
// repositories/database_factory.rs
#[derive(Debug)]
pub enum DatabaseType {
    PostgreSQL(PgPool),
    MySQL(MySqlPool),
    MariaDB(MySqlPool), // MariaDB uses MySQL driver
}

pub struct DatabaseFactory;

impl DatabaseFactory {
    pub async fn create_volume_repository(
        config: &DatabaseConfig,
    ) -> Result<Box<dyn VolumeRepository>, DatabaseError> {
        match config.database_type.as_str() {
            "postgresql" => {
                let pool = PgPoolOptions::new()
                    .max_connections(config.max_connections)
                    .connect(&config.connection_string())
                    .await?;
                Ok(Box::new(PostgresVolumeRepository::new(pool)))
            }
            "mysql" | "mariadb" => {
                let pool = MySqlPoolOptions::new()
                    .max_connections(config.max_connections)
                    .connect(&config.connection_string())
                    .await?;
                Ok(Box::new(MySqlVolumeRepository::new(pool)))
            }
            _ => Err(DatabaseError::UnsupportedDatabase),
        }
    }
}
```

### 6.2 Repositories
```rust
// repositories/volume_repository.rs
#[async_trait]
pub trait VolumeRepository: Send + Sync {
    async fn create(&self, volume: CreateVolumeRequest) -> Result<Volume, RepoError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Volume>, RepoError>;
    async fn find_by_barcode(&self, barcode: &str) -> Result<Option<Volume>, RepoError>;
    async fn search(&self, criteria: SearchCriteria) -> Result<Vec<Volume>, RepoError>;
    async fn update(&self, id: Uuid, volume: UpdateVolumeRequest) -> Result<Volume, RepoError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepoError>;
}

// PostgreSQL implementation
pub struct PostgresVolumeRepository {
    pool: PgPool,
}

impl VolumeRepository for PostgresVolumeRepository {
    async fn create(&self, request: CreateVolumeRequest) -> Result<Volume, RepoError> {
        let volume = sqlx::query_as!(
            Volume,
            r#"
            INSERT INTO volumes (id, title, isbn, publisher, barcode, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
            RETURNING *
            "#,
            Uuid::new_v4(),
            request.title,
            request.isbn,
            request.publisher,
            request.barcode,
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(volume)
    }
}

// MySQL/MariaDB implementation
pub struct MySqlVolumeRepository {
    pool: MySqlPool,
}

impl VolumeRepository for MySqlVolumeRepository {
    async fn create(&self, request: CreateVolumeRequest) -> Result<Volume, RepoError> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            INSERT INTO volumes (id, title, isbn, publisher, barcode, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            id.to_string(),
            request.title,
            request.isbn,
            request.publisher,
            request.barcode,
            now,
            now,
        )
        .execute(&self.pool)
        .await?;
        
        // Fetch created volume
        let volume = sqlx::query_as!(
            Volume,
            "SELECT * FROM volumes WHERE id = ?",
            id.to_string()
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(volume)
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
-- migrations/postgresql/001_initial.sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE volume_condition AS ENUM ('excellent', 'good', 'fair', 'poor', 'lost');
CREATE TYPE loan_status AS ENUM ('active', 'returned', 'overdue', 'lost');

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

CREATE TABLE volumes (
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
    location VARCHAR(255),
    condition volume_condition DEFAULT 'good',
    barcode VARCHAR(50) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_volumes_barcode ON volumes(barcode);
CREATE INDEX idx_volumes_isbn ON volumes(isbn);
CREATE INDEX idx_volumes_title ON volumes USING gin(to_tsvector('english', title));
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

### 8.4 Frontend - Global state management
```rust
// frontend/src/services/state.rs
use leptos::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct AppState {
    pub volumes: RwSignal<Vec<Volume>>,
    pub current_user: RwSignal<Option<User>>,
    pub loading: RwSignal<bool>,
    pub notifications: RwSignal<Vec<Notification>>,
    pub i18n: RwSignal<HashMap<String, String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            volumes: create_rw_signal(Vec::new()),
            current_user: create_rw_signal(None),
            loading: create_rw_signal(false),
            notifications: create_rw_signal(Vec::new()),
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
```rust
// tests/integration/volumes_test.rs
#[tokio::test]
async fn test_create_volume() {
    let app = create_test_app().await;
    let client = TestClient::new(app);
    
    let volume_data = json!({
        "title": "Test Book",
        "isbn": "9781234567890"
    });
    
    let response = client
        .post("/api/v1/volumes")
        .json(&volume_data)
        .send()
        .await;
    
    assert_eq!(response.status(), StatusCode::CREATED);
}
```

### 10.2 Quality tools
- **Unit tests**: cargo test
- **Integration tests**: TestClient with test database
- **Linting**: clippy
- **Formatting**: rustfmt
- **Coverage**: tarpaulin
- **Security**: cargo audit

## 11. Performance and Monitoring

### 11.1 Optimizations
- **Connection pooling**: Configured SQLx pool
- **Redis cache**: for frequent queries
- **Pagination**: default limit on lists
- **Database indexes**: on search columns
- **Compression**: gzip on API responses

### 11.2 Monitoring
```rust
// middleware/metrics.rs
pub async fn metrics_middleware(
    req: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    
    let response = next.run(req).await;
    
    let duration = start.elapsed();
    tracing::info!(
        method = %method,
        path = %path,
        status = %response.status(),
        duration_ms = duration.as_millis(),
        "HTTP request completed"
    );
    
    response
}
```

### 11.3 Scan page with barcode scanner
```rust
// frontend/src/pages/scan.rs
use leptos::*;
use web_sys::HtmlInputElement;

#[component]
pub fn ScanPage() -> impl IntoView {
    let (barcode, set_barcode) = create_signal(String::new());
    let (scan_result, set_scan_result) = create_signal(None::<Volume>);
    let (loading, set_loading) = create_signal(false);
    let app_state = use_app_state();
    let api_client = use_context::<ApiClient>().expect("ApiClient not provided");

    let scan_action = create_action(move |barcode: &String| {
        let barcode = barcode.clone();
        let api_client = api_client.clone();
        async move {
            set_loading.set(true);
            match api_client.scan_barcode(&barcode).await {
                Ok(Some(volume)) => {
                    set_scan_result.set(Some(volume));
                    app_state.add_notification(
                        "Volume found!".to_string(), 
                        NotificationLevel::Success
                    );
                }
                Ok(None) => {
                    set_scan_result.set(None);
                    app_state.add_notification(
                        "No volume found for this barcode".to_string(), 
                        NotificationLevel::Warning
                    );
                }
                Err(e) => {
                    app_state.add_notification(
                        format!("Scan error: {}", e), 
                        NotificationLevel::Error
                    );
                }
            }
            set_loading.set(false);
        }
    });

    let on_scan = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            let barcode_value = barcode.get();
            if !barcode_value.is_empty() {
                scan_action.dispatch(barcode_value);
                set_barcode.set(String::new());
            }
        }
    };

    view! {
        <div class="max-w-2xl mx-auto">
            <h1 class="text-2xl font-bold mb-6">"Scan a barcode"</h1>
            
            <div class="bg-white rounded-lg shadow-md p-6">
                <div class="mb-4">
                    <label class="block text-sm font-medium text-gray-700 mb-2">
                        "Barcode"
                    </label>
                    <input
                        type="text"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        placeholder="Scan or enter barcode..."
                        prop:value=barcode
                        on:input=move |ev| set_barcode.set(event_target_value(&ev))
                        on:keydown=on_scan
                        autofocus
                    />
                </div>

                <Show when=move || loading.get()>
                    <div class="flex items-center justify-center py-4">
                        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
                        <span class="ml-2">"Searching..."</span>
                    </div>
                </Show>

                <Show when=move || scan_result.get().is_some()>
                    {move || scan_result.get().map(|volume| view! {
                        <div class="mt-6 p-4 bg-green-50 border border-green-200 rounded-md">
                            <h3 class="text-lg font-semibold text-green-800">"Volume found!"</h3>
                            <VolumeCard volume=volume />
                            <div class="mt-4 flex space-x-2">
                                <button class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700">
                                    "Loan"
                                </button>
                                <button class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700">
                                    "Return"
                                </button>
                            </div>
                        </div>
                    })}
                </Show>
            </div>
        </div>
    }
}
```

This full-Rust architecture ensures complete technological consistency, excellent performance through WebAssembly, and a unified development experience for your personal library manager.
