# Technical Architecture - Personal Library Manager

> **⚠️ IMPORTANT NOTE - ARCHITECTURE CHANGE (November 2024)**
>
> This document was originally written for a **Leptos (web) + Axum (backend)** architecture.
>
> **The project has transitioned to:**
> - **Frontend**: **Slint** UI framework (native-first, WASM compilation later)
> - **Backend**: REST API using **actix-web** (instead of Axum) + **tokio**
> - **Database**: **MariaDB** (instead of PostgreSQL)
>
> **What remains valid:**
> - Data models (Title, Volume, Loan, Borrower, etc.)
> - Business logic and rules
> - Database schema concepts
> - Title/Volume separation architecture
> - Loan management workflows
> - Duplicate detection strategies
>
> **What is outdated:**
> - Frontend implementation details (Section 1.2, Leptos components)
> - Specific framework references (Axum → actix-web, Leptos → Slint)
> - Trunk build tool (now wasm-pack)
> - Tailwind CSS references (now Slint styling)
>
> **For current architecture**, see:
> - `CLAUDE.md` - Current project overview
> - `development_environment.md` - Slint setup guide
> - `planning.md` - Updated development roadmap
> - `api.md` - Backend API documentation

## Overview
**Web application** developed in Rust with a modular, performant and secure architecture for managing a personal library. The frontend uses Slint UI framework compiled to WebAssembly, and the backend uses actix-web for the REST API.

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
- **Containerization**: Single Docker container (Frontend + Backend)
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
│   └── Cargo.toml
├── frontend/
│   ├── src/
│   │   ├── main.rs
│   │   ├── models.rs
│   │   └── api_client.rs
│   ├── ui/
│   │   ├── app-window.slint
│   │   └── ...
│   ├── Cargo.toml
│   └── index.html
├── Dockerfile          # Unified build for both frontend and backend
├── docker-compose.yml
└── documentation/
```

### 2.2 Layered architecture
```
┌─────────────────────────────────────┐
│      Frontend (Slint WASM)          │
│   (Served by Backend Static Files)  │
├─────────────────────────────────────┤
│          REST API (Actix)           │
├─────────────────────────────────────┤
│         Business Services           │
├─────────────────────────────────────┤
│         Repositories                │
├─────────────────────────────────────┤
│             MariaDB                 │
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

### 4.2 Basic API Routes for Personal Use
```rust
// handlers/titles.rs
pub async fn create_title(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateTitleRequest>,
) -> Result<Json<TitleWithVolumes>, AppError> {
    // Create title and optionally first volume
}

pub async fn get_titles(
    State(app_state): State<AppState>,
    Query(params): Query<TitleQueryParams>,
) -> Result<Json<PaginatedResponse<TitleWithVolumes>>, AppError> {
    // Get titles with volume counts and availability
}

pub async fn add_volume_to_title(
    State(app_state): State<AppState>,
    Path(title_id): Path<Uuid>,
    Json(payload): Json<CreateVolumeRequest>,
) -> Result<Json<Volume>, AppError> {
    // Add new volume copy to existing title
}

// handlers/duplicates.rs
pub async fn get_duplicate_candidates(
    State(app_state): State<AppState>,
    Query(params): Query<DuplicateQueryParams>,
) -> Result<Json<Vec<DuplicateCandidate>>, AppError> {
    // Get potential duplicates for review
}

pub async fn merge_titles(
    State(app_state): State<AppState>,
    Json(payload): Json<MergeTitlesRequest>,
) -> Result<Json<TitleWithVolumes>, AppError> {
    // Merge two duplicate titles
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

## 5. Services and Business Logic

### 5.1 Title/Volume Service Architecture
```rust
// services/title_service.rs
pub struct TitleService {
    title_repo: Arc<dyn TitleRepository>,
    volume_repo: Arc<dyn VolumeRepository>,
    author_repo: Arc<dyn AuthorRepository>,
    barcode_generator: Arc<dyn BarcodeGenerator>,
    duplicate_service: Arc<DuplicateService>,
}

impl TitleService {
    pub async fn create_title_with_volume(
        &self,
        request: CreateTitleRequest,
    ) -> Result<TitleWithVolumes, ServiceError> {
        // 1. Check for duplicates before creating
        let duplicates = self.duplicate_service
            .detect_duplicates_for_new_title(&request)
            .await?;
        
        if !duplicates.is_empty() && !request.force_create {
            return Err(ServiceError::PotentialDuplicates(duplicates));
        }
        
        // 2. Create the title
        let mut title_data = request.into();
        // Set title type for loan duration rules
        if title_data.title_type.is_none() {
            title_data.title_type = Some(self.infer_title_type(&title_data));
        }
        
        let title = self.title_repo.create(title_data).await?;
        
        // 3. Create first volume automatically if requested
        if request.create_first_volume {
            let volume_request = CreateVolumeRequest {
                title_id: title.id,
                copy_number: 1,
                condition: VolumeCondition::Good,
                location: request.initial_location,
            };
            
            let volume = self.volume_repo.create(volume_request).await?;
            
            Ok(TitleWithVolumes {
                title,
                volumes: vec![volume],
                authors: vec![],
                total_volumes: 1,
                available_volumes: 1,
            })
        } else {
            // Wishlist title with 0 volumes
            Ok(TitleWithVolumes {
                title,
                volumes: vec![],
                authors: vec![],
                total_volumes: 0,
                available_volumes: 0,
            })
        }
    }

    fn infer_title_type(&self, title_data: &CreateTitleData) -> TitleType {
        // Simple inference based on genre or Dewey code
        if let Some(dewey) = &title_data.dewey_code {
            match dewey.chars().next() {
                Some('0'..='1') => TitleType::Reference,
                Some('8') => TitleType::Fiction,
                _ => TitleType::NonFiction,
            }
        } else if let Some(genre) = &title_data.genre {
            match genre.to_lowercase().as_str() {
                "fiction" | "novel" | "fantasy" | "sci-fi" => TitleType::Fiction,
                "magazine" | "periodical" => TitleType::Magazine,
                "reference" | "dictionary" | "encyclopedia" => TitleType::Reference,
                _ => TitleType::NonFiction,
            }
        } else {
            TitleType::NonFiction
        }
    }

    pub async fn add_volume_copy(
        &self,
        title_id: Uuid,
        request: AddVolumeCopyRequest,
    ) -> Result<Volume, ServiceError> {
        // Find the next copy number
        let next_copy_number = self.volume_repo
            .get_next_copy_number(title_id)
            .await?;
        
        let volume_request = CreateVolumeRequest {
            title_id,
            copy_number: next_copy_number,
            condition: request.condition,
            location: request.location,
        };
        
        self.volume_repo.create(volume_request).await
    }

    pub async fn get_title_with_volumes(
        &self,
        title_id: Uuid,
    ) -> Result<TitleWithVolumes, ServiceError> {
        let title = self.title_repo.find_by_id(title_id).await?
            .ok_or(ServiceError::NotFound)?;
        
        let volumes = self.volume_repo.find_by_title_id(title_id).await?;
        let authors = self.author_repo.find_by_title_id(title_id).await?;
        
        let available_volumes = volumes.iter()
            .filter(|v| v.loan_status == LoanStatus::Available)
            .count() as i32;
        
        Ok(TitleWithVolumes {
            title,
            volumes: volumes.clone(),
            authors,
            total_volumes: volumes.len() as i32,
            available_volumes,
        })
    }
}

// services/loan_service.rs (Simplified for personal use)
pub struct LoanService {
    loan_repo: Arc<dyn LoanRepository>,
    volume_repo: Arc<dyn VolumeRepository>,
    title_repo: Arc<dyn TitleRepository>,
    wishlist_repo: Arc<dyn WishlistRepository>,
}

impl LoanService {
    pub async fn create_loan(
        &self,
        title_id: Uuid,
        borrower_id: Uuid,
    ) -> Result<Loan, ServiceError> {
        // 1. Find best available volume
        let volume = self.select_best_available_volume(title_id).await?
            .ok_or(ServiceError::NoVolumeAvailable)?;
        
        // 2. Get title to determine loan duration
        let title = self.title_repo.find_by_id(title_id).await?
            .ok_or(ServiceError::NotFound)?;
        
        let loan_days = title.title_type
            .map(|t| t.default_loan_days())
            .unwrap_or(14);
        
        // 3. Create loan
        let loan_request = CreateLoanRequest {
            title_id,
            volume_id: volume.id,
            borrower_id,
            due_date: Utc::now() + Duration::days(loan_days as i64),
        };
        
        let loan = self.loan_repo.create(loan_request).await?;
        
        // 4. Update volume status
        self.volume_repo.update_loan_status(volume.id, LoanStatus::Loaned).await?;
        
        // 5. Check if anyone has this title on wishlist
        self.notify_wishlist_users(title_id).await?;
        
        Ok(loan)
    }

    async fn select_best_available_volume(
        &self,
        title_id: Uuid,
    ) -> Result<Option<Volume>, ServiceError> {
        let available_volumes = self.volume_repo
            .find_available_by_title_id(title_id)
            .await?;
        
        if available_volumes.is_empty() {
            return Ok(None);
        }
        
        // Selection priority:
        // 1. Best physical condition
        // 2. Lowest copy number (FIFO)
        let best_volume = available_volumes
            .into_iter()
            .min_by(|a, b| {
                let condition_order = |c: &VolumeCondition| match c {
                    VolumeCondition::Excellent => 0,
                    VolumeCondition::Good => 1,
                    VolumeCondition::Fair => 2,
                    VolumeCondition::Poor => 3,
                    VolumeCondition::Damaged => 4,
                };
                
                condition_order(&a.condition)
                    .cmp(&condition_order(&b.condition))
                    .then_with(|| a.copy_number.cmp(&b.copy_number))
            });
        
        Ok(best_volume)
    }

    async fn notify_wishlist_users(&self, title_id: Uuid) -> Result<(), ServiceError> {
        // Simple notification for wishlist users when title becomes available
        let wishlist_items = self.wishlist_repo.find_by_title_id(title_id).await?;
        
        for item in wishlist_items {
            // Send basic email notification if configured
            // This is optional and simple
        }
        
        Ok(())
    }
}

// services/duplicate_service.rs (New service for duplicate management)
pub struct DuplicateService {
    duplicate_repo: Arc<dyn DuplicateRepository>,
    title_repo: Arc<dyn TitleRepository>,
}

impl DuplicateService {
    pub async fn detect_duplicates_for_new_title(
        &self,
        request: &CreateTitleRequest,
    ) -> Result<Vec<DuplicateCandidate>, ServiceError> {
        let mut candidates = Vec::new();
        
        // 1. Check for identical ISBN
        if let Some(isbn) = &request.isbn {
            if let Some(existing) = self.title_repo.find_by_isbn(isbn).await? {
                candidates.push(DuplicateCandidate {
                    id: Uuid::new_v4(),
                    title_id_1: Uuid::new_v4(), // Would be the new title's ID
                    title_id_2: existing.id,
                    confidence_score: 1.0,
                    detection_type: DuplicateType::IdenticalIsbn,
                    status: DuplicateStatus::Pending,
                    created_at: Utc::now(),
                    reviewed_at: None,
                });
            }
        }
        
        // 2. Check for title + author matches
        let similar_titles = self.title_repo
            .find_similar_titles(&request.title, &request.authors)
            .await?;
        
        for similar in similar_titles {
            let confidence = self.calculate_similarity_score(&request.title, &similar.title);
            if confidence > 0.85 {
                candidates.push(DuplicateCandidate {
                    id: Uuid::new_v4(),
                    title_id_1: Uuid::new_v4(),
                    title_id_2: similar.id,
                    confidence_score: confidence,
                    detection_type: if confidence > 0.95 {
                        DuplicateType::TitleAuthorMatch
                    } else {
                        DuplicateType::FuzzyMatch
                    },
                    status: DuplicateStatus::Pending,
                    created_at: Utc::now(),
                    reviewed_at: None,
                });
            }
        }
        
        Ok(candidates)
    }

    pub async fn merge_titles(
        &self,
        primary_id: Uuid,
        secondary_id: Uuid,
    ) -> Result<TitleWithVolumes, ServiceError> {
        // 1. Get both titles
        let primary = self.title_repo.find_by_id(primary_id).await?
            .ok_or(ServiceError::NotFound)?;
        let secondary = self.title_repo.find_by_id(secondary_id).await?
            .ok_or(ServiceError::NotFound)?;
        
        // 2. Merge metadata (keep primary, add secondary info as needed)
        let mut merged_title = primary;
        if merged_title.isbn.is_none() && secondary.isbn.is_some() {
            merged_title.isbn = secondary.isbn;
        }
        if merged_title.cover_url.is_none() && secondary.cover_url.is_some() {
            merged_title.cover_url = secondary.cover_url;
        }
        
        // 3. Move all volumes from secondary to primary
        self.title_repo.move_volumes_to_title(secondary_id, primary_id).await?;
        
        // 4. Update copy numbers sequentially
        self.title_repo.renumber_volumes(primary_id).await?;
        
        // 5. Delete secondary title
        self.title_repo.delete(secondary_id).await?;
        
        // 6. Mark duplicate as merged
        self.duplicate_repo.mark_as_merged(primary_id, secondary_id).await?;
        
        // 7. Return merged result
        let volumes = self.title_repo.get_volumes_for_title(primary_id).await?;
        let authors = self.title_repo.get_authors_for_title(primary_id).await?;
        
        Ok(TitleWithVolumes {
            title: merged_title,
            volumes: volumes.clone(),
            authors,
            total_volumes: volumes.len() as i32,
            available_volumes: volumes.iter()
                .filter(|v| v.loan_status == LoanStatus::Available)
                .count() as i32,
        })
    }

    fn calculate_similarity_score(&self, title1: &str, title2: &str) -> f32 {
        // Simple Levenshtein distance implementation
        let title1_normalized = self.normalize_string(title1);
        let title2_normalized = self.normalize_string(title2);
        
        let distance = levenshtein_distance(&title1_normalized, &title2_normalized);
        let max_len = title1_normalized.len().max(title2_normalized.len());
        
        if max_len == 0 {
            1.0
        } else {
            1.0 - (distance as f32 / max_len as f32)
        }
    }

    fn normalize_string(&self, s: &str) -> String {
        s.to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }
}

// services/wishlist_service.rs (Simple wishlist instead of complex reservations)
pub struct WishlistService {
    wishlist_repo: Arc<dyn WishlistRepository>,
    title_repo: Arc<dyn TitleRepository>,
}

impl WishlistService {
    pub async fn add_to_wishlist(
        &self,
        title_id: Uuid,
        borrower_id: Uuid,
        notes: Option<String>,
    ) -> Result<WishlistItem, ServiceError> {
        // Check if title exists
        self.title_repo.find_by_id(title_id).await?
            .ok_or(ServiceError::NotFound)?;
        
        let wishlist_item = WishlistItem {
            id: Uuid::new_v4(),
            title_id,
            borrower_id,
            added_date: Utc::now(),
            notes,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.wishlist_repo.create(wishlist_item).await
    }

    pub async fn get_borrower_wishlist(
        &self,
        borrower_id: Uuid,
    ) -> Result<Vec<WishlistItemWithTitle>, ServiceError> {
        self.wishlist_repo.find_by_borrower_with_titles(borrower_id).await
    }
}
```

### 5.2 Barcode and Scanner Services
```rust
// services/barcode_service.rs
pub trait BarcodeGenerator: Send + Sync {
    async fn generate_unique_barcode(&self) -> Result<String, BarcodeError>;
    fn validate_volume_barcode(&self, barcode: &str) -> bool;
    fn validate_isbn_barcode(&self, isbn: &str) -> bool;
}

pub struct SequentialBarcodeGenerator {
    repo: Arc<dyn BarcodeRepository>,
}

impl BarcodeGenerator for SequentialBarcodeGenerator {
    async fn generate_unique_barcode(&self) -> Result<String, BarcodeError> {
        let next_id = self.repo.get_next_sequence().await?;
        Ok(format!("VOL-{:06}", next_id)) // Code 128 format
    }
    
    fn validate_volume_barcode(&self, barcode: &str) -> bool {
        // Validate VOL-XXXXXX format (Code 128)
        let re = regex::Regex::new(r"^VOL-\d{6}$").unwrap();
        re.is_match(barcode)
    }
    
    fn validate_isbn_barcode(&self, isbn: &str) -> bool {
        // Validate EAN-13 format for ISBN
        let re = regex::Regex::new(r"^\d{13}$").unwrap();
        re.is_match(isbn) && self.validate_isbn_checksum(isbn)
    }
    
    fn validate_isbn_checksum(&self, isbn: &str) -> bool {
        // EAN-13 checksum validation
        let digits: Vec<u32> = isbn.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        
        if digits.len() != 13 {
            return false;
        }
        
        let sum: u32 = digits[..12].iter()
            .enumerate()
            .map(|(i, &digit)| if i % 2 == 0 { digit } else { digit * 3 })
            .sum();
        
        let check_digit = (10 - (sum % 10)) % 10;
        check_digit == digits[12]
    }
}

// services/scanner_service.rs (Enhanced for dual barcode support)
pub struct ScannerService {
    volume_repo: Arc<dyn VolumeRepository>,
    title_repo: Arc<dyn TitleRepository>,
    barcode_generator: Arc<dyn BarcodeGenerator>,
    external_api: Arc<dyn ExternalMetadataService>,
}

impl ScannerService {
    pub async fn scan_barcode(&self, barcode: &str) -> Result<ScanResult, ServiceError> {
        // Determine barcode type and handle accordingly
        if self.barcode_generator.validate_volume_barcode(barcode) {
            self.scan_volume_barcode(barcode).await
        } else if self.barcode_generator.validate_isbn_barcode(barcode) {
            self.scan_isbn_barcode(barcode).await
        } else {
            Err(ServiceError::InvalidBarcode)
        }
    }
    
    async fn scan_volume_barcode(&self, barcode: &str) -> Result<ScanResult, ServiceError> {
        let volume_with_context = self.volume_repo
            .find_with_title_context(barcode)
            .await?
            .ok_or(ServiceError::VolumeNotFound)?;
        
        Ok(ScanResult::Volume(volume_with_context))
    }
    
    async fn scan_isbn_barcode(&self, isbn: &str) -> Result<ScanResult, ServiceError> {
        // First check if we have this title in our collection
        if let Some(title) = self.title_repo.find_by_isbn(isbn).await? {
            let volumes = self.volume_repo.find_by_title_id(title.id).await?;
            Ok(ScanResult::ExistingTitle(TitleWithVolumes {
                title,
                volumes: volumes.clone(),
                authors: vec![], // Would be populated in real implementation
                total_volumes: volumes.len() as i32,
                available_volumes: volumes.iter()
                    .filter(|v| v.loan_status == LoanStatus::Available)
                    .count() as i32,
            }))
        } else {
            // Try to fetch metadata from external API
            match self.external_api.get_metadata_by_isbn(isbn).await {
                Ok(metadata) => Ok(ScanResult::NewTitleMetadata(metadata)),
                Err(_) => Ok(ScanResult::UnknownIsbn(isbn.to_string())),
            }
        }
    }
}

#[derive(Debug)]
pub enum ScanResult {
    Volume(VolumeWithTitleContext),
    ExistingTitle(TitleWithVolumes),
    NewTitleMetadata(ExternalMetadata),
    UnknownIsbn(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeWithTitleContext {
    pub volume: Volume,
    pub title: Title,
    pub other_copies: Vec<Volume>,
    pub authors: Vec<Author>,
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

### 6.2 Simplified Repositories for Personal Use
```rust
// repositories/title_repository.rs
#[async_trait]
pub trait TitleRepository: Send + Sync {
    async fn create(&self, title: CreateTitleRequest) -> Result<Title, RepoError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Title>, RepoError>;
    async fn find_by_isbn(&self, isbn: &str) -> Result<Option<Title>, RepoError>;
    async fn find_similar_titles(&self, title: &str, authors: &[String]) -> Result<Vec<Title>, RepoError>;
    async fn search(&self, criteria: TitleSearchCriteria) -> Result<Vec<Title>, RepoError>;
    async fn update(&self, id: Uuid, title: UpdateTitleRequest) -> Result<Title, RepoError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepoError>;
    async fn find_wishlist(&self) -> Result<Vec<Title>, RepoError>;
    async fn get_titles_with_volumes(&self, params: TitleQueryParams) -> Result<PaginatedResponse<TitleWithVolumes>, RepoError>;
    // For duplicate management
    async fn move_volumes_to_title(&self, from_title_id: Uuid, to_title_id: Uuid) -> Result<(), RepoError>;
    async fn renumber_volumes(&self, title_id: Uuid) -> Result<(), RepoError>;
    async fn get_volumes_for_title(&self, title_id: Uuid) -> Result<Vec<Volume>, RepoError>;
    async fn get_authors_for_title(&self, title_id: Uuid) -> Result<Vec<Author>, RepoError>;
}

// repositories/volume_repository.rs
#[async_trait]
pub trait VolumeRepository: Send + Sync {
    async fn create(&self, volume: CreateVolumeRequest) -> Result<Volume, RepoError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Volume>, RepoError>;
    async fn find_by_barcode(&self, barcode: &str) -> Result<Option<Volume>, RepoError>;
    async fn find_by_title_id(&self, title_id: Uuid) -> Result<Vec<Volume>, RepoError>;
    async fn find_available_by_title_id(&self, title_id: Uuid) -> Result<Vec<Volume>, RepoError>;
    async fn get_next_copy_number(&self, title_id: Uuid) -> Result<i32, RepoError>;
    async fn update_loan_status(&self, id: Uuid, status: LoanStatus) -> Result<(), RepoError>;
    async fn search(&self, criteria: VolumeSearchCriteria) -> Result<Vec<Volume>, RepoError>;
    async fn update(&self, id: Uuid, volume: UpdateVolumeRequest) -> Result<Volume, RepoError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepoError>;
    // Enhanced scanner support
    async fn find_with_title_context(&self, barcode: &str) -> Result<Option<VolumeWithTitleContext>, RepoError>;
}

// repositories/borrower_repository.rs (Simplified)
#[async_trait]
pub trait BorrowerRepository: Send + Sync {
    async fn create(&self, borrower: CreateBorrowerRequest) -> Result<Borrower, RepoError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Borrower>, RepoError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<Borrower>, RepoError>;
    async fn search(&self, query: &str) -> Result<Vec<Borrower>, RepoError>;
    async fn update(&self, id: Uuid, borrower: UpdateBorrowerRequest) -> Result<Borrower, RepoError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepoError>;
}

// repositories/wishlist_repository.rs (Simple wishlist instead of reservations)
#[async_trait]
pub trait WishlistRepository: Send + Sync {
    async fn create(&self, item: WishlistItem) -> Result<WishlistItem, RepoError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<WishlistItem>, RepoError>;
    async fn find_by_borrower_id(&self, borrower_id: Uuid) -> Result<Vec<WishlistItem>, RepoError>;
    async fn find_by_title_id(&self, title_id: Uuid) -> Result<Vec<WishlistItem>, RepoError>;
    async fn find_by_borrower_with_titles(&self, borrower_id: Uuid) -> Result<Vec<WishlistItemWithTitle>, RepoError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepoError>;
}

// repositories/duplicate_repository.rs (New for duplicate management)
#[async_trait]
pub trait DuplicateRepository: Send + Sync {
    async fn create(&self, candidate: DuplicateCandidate) -> Result<DuplicateCandidate, RepoError>;
    async fn find_pending(&self) -> Result<Vec<DuplicateCandidate>, RepoError>;
    async fn find_by_title_id(&self, title_id: Uuid) -> Result<Vec<DuplicateCandidate>, RepoError>;
    async fn update_status(&self, id: Uuid, status: DuplicateStatus) -> Result<(), RepoError>;
    async fn mark_as_merged(&self, primary_id: Uuid, secondary_id: Uuid) -> Result<(), RepoError>;
    async fn mark_as_ignored(&self, id: Uuid) -> Result<(), RepoError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepoError>;
}

// repositories/loan_repository.rs (Simplified)
#[async_trait]
pub trait LoanRepository: Send + Sync {
    async fn create(&self, loan: CreateLoanRequest) -> Result<Loan, RepoError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Loan>, RepoError>;
    async fn find_active(&self) -> Result<Vec<Loan>, RepoError>;
    async fn find_overdue(&self) -> Result<Vec<Loan>, RepoError>;
    async fn find_by_borrower_id(&self, borrower_id: Uuid) -> Result<Vec<Loan>, RepoError>;
    async fn find_by_volume_id(&self, volume_id: Uuid) -> Result<Option<Loan>, RepoError>;
    async fn return_loan(&self, id: Uuid) -> Result<Loan, RepoError>;
    async fn extend_loan(&self, id: Uuid, new_due_date: DateTime<Utc>) -> Result<Loan, RepoError>;
}
```

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

### 11.3 Enhanced Scan Page with Dual Barcode Support
```rust
// frontend/src/pages/scan.rs
use leptos::*;
use web_sys::HtmlInputElement;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanResult {
    Volume(VolumeWithTitleContext),
    ExistingTitle(TitleWithVolumes),
    NewTitleMetadata(ExternalMetadata),
    UnknownIsbn(String),
}

#[component]
pub fn ScanPage() -> impl IntoView {
    let (barcode, set_barcode) = create_signal(String::new());
    let (scan_result, set_scan_result) = create_signal(None::<ScanResult>);
    let (loading, set_loading) = create_signal(false);
    let app_state = use_app_state();
    let api_client = use_context::<ApiClient>().expect("ApiClient not provided");

    let scan_action = create_action(move |barcode: &String| {
        let barcode = barcode.clone();
        let api_client = api_client.clone();
        async move {
            set_loading.set(true);
            match api_client.scan_barcode(&barcode).await {
                Ok(result) => {
                    set_scan_result.set(Some(result));
                    match &result {
                        ScanResult::Volume(_) => app_state.add_notification(
                            "Volume found!".to_string(), 
                            NotificationLevel::Success
                        ),
                        ScanResult::ExistingTitle(_) => app_state.add_notification(
                            "Title found in collection!".to_string(), 
                            NotificationLevel::Success
                        ),
                        ScanResult::NewTitleMetadata(_) => app_state.add_notification(
                            "New title metadata retrieved!".to_string(), 
                            NotificationLevel::Info
                        ),
                        ScanResult::UnknownIsbn(_) => app_state.add_notification(
                            "Unknown ISBN - you can add it manually".to_string(), 
                            NotificationLevel::Warning
                        ),
                    }
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
        <div class="max-w-4xl mx-auto">
            <h1 class="text-2xl font-bold mb-6">"Barcode Scanner"</h1>
            
            <div class="bg-white rounded-lg shadow-md p-6">
                <div class="mb-4">
                    <label class="block text-sm font-medium text-gray-700 mb-2">
                        "Barcode (Volume or ISBN)"
                    </label>
                    <input
                        type="text"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        placeholder="Scan volume barcode (VOL-XXXXXX) or ISBN (13 digits)..."
                        prop:value=barcode
                        on:input=move |ev| set_barcode.set(event_target_value(&ev))
                        on:keydown=on_scan
                        autofocus
                    />
                    <p class="text-xs text-gray-500 mt-1">
                        "Volume barcodes (Code 128) for loan/return operations, ISBN barcodes (EAN-13) for title lookup/add"
                    </p>
                </div>

                <Show when=move || loading.get()>
                    <div class="flex items-center justify-center py-4">
                        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
                        <span class="ml-2">"Searching..."</span>
                    </div>
                </Show>

                <Show when=move || scan_result.get().is_some()>
                    {move || scan_result.get().map(|result| match result {
                        ScanResult::Volume(volume_context) => view! {
                            <VolumeResultDisplay volume_context=volume_context />
                        }.into_view(),
                        ScanResult::ExistingTitle(title_with_volumes) => view! {
                            <ExistingTitleDisplay title_with_volumes=title_with_volumes />
                        }.into_view(),
                        ScanResult::NewTitleMetadata(metadata) => view! {
                            <NewTitleDisplay metadata=metadata />
                        }.into_view(),
                        ScanResult::UnknownIsbn(isbn) => view! {
                            <UnknownIsbnDisplay isbn=isbn />
                        }.into_view(),
                    })}
                </Show>
            </div>
        </div>
    }
}

#[component]
fn VolumeResultDisplay(volume_context: VolumeWithTitleContext) -> impl IntoView {
    view! {
        <div class="mt-6 space-y-6">
            // Title context
            <div class="p-4 bg-blue-50 border border-blue-200 rounded-md">
                <h3 class="text-lg font-semibold text-blue-800 mb-2">
                    {format!("Volume {}/{} of \"{}\"", 
                        volume_context.volume.copy_number,
                        volume_context.other_copies.len() + 1,
                        volume_context.title.title
                    )}
                </h3>
                // ... rest of volume display
            </div>
            
            // Actions for volume
            <div class="flex space-x-4">
                <Show when=move || volume_context.volume.loan_status == LoanStatus::Available>
                    <button class="px-6 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 font-medium">
                        "Loan This Volume"
                    </button>
                </Show>
                <Show when=move || volume_context.volume.loan_status == LoanStatus::Loaned>
                    <button class="px-6 py-2 bg-green-600 text-white rounded hover:bg-green-700 font-medium">
                        "Return This Volume"
                    </button>
                </Show>
            </div>
        </div>
    }
}

#[component]
fn ExistingTitleDisplay(title_with_volumes: TitleWithVolumes) -> impl IntoView {
    view! {
        <div class="mt-6 space-y-6">
            <div class="p-4 bg-green-50 border border-green-200 rounded-md">
                <h3 class="text-lg font-semibold text-green-800 mb-2">
                    {format!("\"{}\" - Found in Collection", title_with_volumes.title.title)}
                </h3>
                <p class="text-sm text-gray-600">
                    {format!("{} copies total, {} available", 
                        title_with_volumes.total_volumes,
                        title_with_volumes.available_volumes
                    )}
                </p>
            </div>
            
            // Actions for existing title
            <div class="flex space-x-4">
                <button class="px-6 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 font-medium">
                    "View Title Details"
                </button>
                <Show when=move || title_with_volumes.available_volumes > 0>
                    <button class="px-6 py-2 bg-green-600 text-white rounded hover:bg-green-700 font-medium">
                        "Loan Available Copy"
                    </button>
                </Show>
                <button class="px-6 py-2 bg-purple-600 text-white rounded hover:bg-purple-700 font-medium">
                    "Add Another Copy"
                </button>
            </div>
        </div>
    }
}

#[component]
fn NewTitleDisplay(metadata: ExternalMetadata) -> impl IntoView {
    view! {
        <div class="mt-6 space-y-6">
            <div class="p-4 bg-yellow-50 border border-yellow-200 rounded-md">
                <h3 class="text-lg font-semibold text-yellow-800 mb-2">
                    {format!("New Title: \"{}\"", metadata.title)}
                </h3>
                <p class="text-sm text-gray-600 mb-2">
                    {format!("Author: {}", metadata.authors.join(", "))}
                </p>
                <p class="text-sm text-gray-600">
                    {format!("Publisher: {} ({})", 
                        metadata.publisher.unwrap_or_else(|| "Unknown".to_string()),
                        metadata.publication_year.map(|y| y.to_string()).unwrap_or_else(|| "Unknown".to_string())
                    )}
                </p>
            </div>
            
            // Actions for new title
            <div class="flex space-x-4">
                <button class="px-6 py-2 bg-green-600 text-white rounded hover:bg-green-700 font-medium">
                    "Add to Collection"
                </button>
                <button class="px-6 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 font-medium">
                    "Add to Wishlist"
                </button>
            </div>
        </div>
    }
}

#[component]
fn UnknownIsbnDisplay(isbn: String) -> impl IntoView {
    view! {
        <div class="mt-6 space-y-6">
            <div class="p-4 bg-gray-50 border border-gray-200 rounded-md">
                <h3 class="text-lg font-semibold text-gray-800 mb-2">
                    "Unknown ISBN"
                </h3>
                <p class="text-sm text-gray-600 mb-2">
                    {format!("ISBN: {}", isbn)}
                </p>
                <p class="text-sm text-gray-600">
                    "This ISBN was not found in external databases. You can add it manually."
                </p>
            </div>
            
            // Actions for unknown ISBN
            <div class="flex space-x-4">
                <button class="px-6 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 font-medium">
                    "Add Manually"
                </button>
            </div>
        </div>
    }
}
```

This full-Rust architecture ensures complete technological consistency, excellent performance through WebAssembly, and a unified development experience for your personal library manager.
