# Development Planning - Personal Library Manager (rbibli)

## Architecture Overview

The project uses a **client-server architecture**:
- **Frontend**: Slint UI framework (native desktop, WASM later)
- **Backend**: REST API using actix-web + tokio

**Development Approach**: Native-first development for faster iteration, with WASM compilation planned for later. This allows:
- Fast compile-test cycles during development
- Native performance and debugging
- Browser-based access later from any device
- Independent development of UI and business logic
- Potential future native/mobile apps using the same backend
- Standard web deployment patterns

## Current Status (Updated: 2025-01-13)

### Frontend (Slint Native UI)
- ✅ Basic Slint project structure created
- ✅ Sidebar navigation component (8 menu items)
- ✅ Page routing structure with conditional rendering
- ✅ **ScrollView** for responsive content areas
- ✅ **Multiple UI pages implemented:**
  - ✅ Titles Page (create, edit, list with genre dropdown)
  - ✅ Authors Page (full CRUD)
  - ✅ Publishers Page (full CRUD)
  - ✅ Genres Page (full CRUD)
  - ✅ Locations Page (full CRUD with hierarchical structure)
  - ✅ About Page
- ✅ **Data models** (models.rs): Title, Author, Publisher, Genre, Location
- ✅ **HTTP client** (api_client.rs): reqwest-based API client
- ✅ **Full integration with backend API** via callbacks
- ✅ Modal dialogs for create/edit operations
- ⏳ WASM build configuration (planned for later)
- ⏳ Volumes page (database ready, implementation needed)
- ⏳ Loans page (database ready, implementation needed)
- ⏳ Scanner interface (not started)

### Backend (actix-web API + MariaDB)
- ✅ actix-web project structure with proper routing
- ✅ Tokio async runtime configured
- ✅ **MariaDB integration** with SQLx (13 migrations applied)
- ✅ **Connection pooling** (MySqlPoolOptions, max 5 connections)
- ✅ **Health check endpoints** (/health, /health/db)
- ✅ **API endpoints implemented:**
  - ✅ Titles: GET, POST, PUT (DELETE missing)
  - ✅ Authors: Full CRUD (GET, POST, PUT, DELETE)
  - ✅ Publishers: Full CRUD
  - ✅ Genres: Full CRUD
  - ✅ Locations: Full CRUD with recursive CTEs for paths
- ✅ **Data repositories** for all implemented entities
- ✅ **Database schema complete** for:
  - ✅ titles, volumes, authors, publishers, genres, locations
  - ✅ title_authors (junction table)
  - ✅ borrowers, loans
- ⏳ Volume API endpoints (database ready, handlers needed)
- ⏳ Borrower/Loan API endpoints (database ready, handlers needed)
- ⏳ Title-Author relationship handlers

## Development Phases

### Phase 1: Core Infrastructure ✅ **COMPLETED**

#### Frontend Tasks
- [x] Complete page structure for main sections
  - [x] **Titles page** (create, edit, list)
  - [x] **Authors page** (full CRUD)
  - [x] **Publishers page** (full CRUD)
  - [x] **Genres page** (full CRUD)
  - [x] **Locations page** (full CRUD with hierarchy)
  - [x] **About page**
  - [ ] Volumes page (pending)
  - [ ] Loans page (pending)
  - [ ] Scanner page (pending)
  - [ ] Statistics page (pending)
- [x] Define shared data models (Title, Author, Publisher, Genre, Location)
  - [ ] Volume, Loan, Borrower models (pending)
- [x] Implement state management with Slint properties and callbacks
- [x] Create reusable UI components
  - [x] Base Page component
  - [x] Modal dialogs for create/edit
  - [x] Sidebar navigation
  - [ ] Volume list component (pending)
  - [ ] Loan card component (pending)
  - [ ] Search bar component (pending)

#### Backend Tasks
- [x] MariaDB database integration with SQLx
- [x] Define complete database schema (13 migrations)
  - [x] titles table (with publisher_id, genre_id FKs)
  - [x] volumes table (with location_id FK, barcode, condition, loan_status)
  - [x] borrowers table
  - [x] loans table (with title_id, volume_id, borrower_id FKs)
  - [x] authors table
  - [x] publishers table
  - [x] genres table
  - [x] locations table (self-referencing hierarchy)
  - [x] title_authors junction table
- [x] Set up sqlx with MariaDB feature and runtime
- [x] Database migrations system (sqlx-cli)
- [x] Implement repository pattern for all entities
- [x] Create CRUD endpoints for core entities
- [x] Health check endpoints (/health, /health/db)

### Phase 2: Title and Volume Management 🔄 **80% COMPLETE**

#### Frontend
- [x] Title management interface
  - [x] List view with volume counts
  - [x] Add new title form (all fields including genre dropdown)
  - [x] Edit title form
  - [ ] Search/filter functionality (pending)
  - [ ] Title detail view (pending)
  - [ ] Title deletion (backend missing)
- [ ] Volume management interface **← CRITICAL PATH**
  - [ ] Add volume to title
  - [ ] List volumes for a title
  - [ ] Edit volume details
  - [ ] Display volume status (available/loaned)
  - [ ] Volume condition tracking
  - [ ] Location assignment

#### Backend
- [x] Title CRUD API **mostly complete**
  - [x] `GET /api/v1/titles` - List titles with volume counts
  - [x] `POST /api/v1/titles` - Create title
  - [x] `GET /api/v1/titles/{id}` - Get title details
  - [x] `PUT /api/v1/titles/{id}` - Update title (partial updates)
  - [ ] `DELETE /api/v1/titles/{id}` - Delete title **← MISSING**
- [ ] Volume CRUD API **← CRITICAL PATH** (database ready, handlers needed)
  - [ ] `POST /api/v1/titles/{id}/volumes` - Add volume
  - [ ] `GET /api/v1/volumes` - List all volumes
  - [ ] `GET /api/v1/volumes/{id}` - Get volume
  - [ ] `PUT /api/v1/volumes/{id}` - Update volume
  - [ ] `DELETE /api/v1/volumes/{id}` - Delete volume
- [ ] Barcode generation service
  - [ ] Sequential barcode generator (VOL-000001)
  - [ ] Uniqueness validation

### Phase 3: Author/Publisher/Genre/Location Management ✅ **COMPLETED**

**Note:** This phase was completed ahead of schedule to establish the full data model.

#### Frontend
- [x] Authors Page (full CRUD with biographical info, title counts)
- [x] Publishers Page (full CRUD with company details, title counts)
- [x] Genres Page (full CRUD with title counts)
- [x] Locations Page (full CRUD with hierarchical paths)
- [x] Genre dropdown in title forms
- [x] Parent location dropdown for hierarchical locations

#### Backend
- [x] Authors API (full CRUD)
- [x] Publishers API (full CRUD)
- [x] Genres API (full CRUD)
- [x] Locations API (full CRUD with recursive CTEs for path building)
- [x] Database migrations for all entities
- [x] Foreign key relationships (titles.publisher_id, titles.genre_id, volumes.location_id)

### Phase 4: Barcode Scanning ⏳ **NOT STARTED**

#### Frontend
- [ ] Scanner interface
  - [ ] Barcode input field (supports scanner devices)
  - [ ] Manual barcode entry
  - [ ] Display scanned item details
  - [ ] Quick actions (loan/return)
- [ ] Dual barcode support
  - [ ] Volume barcode (Code 128) - for operations
  - [ ] ISBN barcode (EAN-13) - for title lookup

#### Backend
- [ ] Scan endpoints
  - [ ] `GET /api/v1/scan/volume/{barcode}` - Find by volume barcode
  - [ ] `GET /api/v1/scan/isbn/{isbn}` - Find by ISBN
- [ ] Barcode validation
  - [ ] Code 128 format validation
  - [ ] EAN-13 checksum validation

### Phase 5: Loan Management 🔄 **10% COMPLETE**

**Status:** Database schema complete, implementation needed.

#### Frontend
- [ ] Borrower management
  - [ ] Add/edit borrower
  - [ ] List borrowers
  - [ ] Search borrowers
- [ ] Loan operations
  - [ ] Create loan interface (title search)
  - [ ] Return volume interface
  - [ ] View active loans
  - [ ] View loan history
- [ ] Loan status indicators
  - [ ] Available/loaned badges
  - [ ] Overdue warnings

#### Backend
- [x] Database schema (borrowers, loans tables)
- [ ] Borrower API
  - [ ] CRUD operations for borrowers
- [ ] Loan API
  - [ ] `POST /api/v1/loans` - Create loan
  - [ ] `GET /api/v1/loans` - List loans
  - [ ] `GET /api/v1/loans/active` - Active loans
  - [ ] `GET /api/v1/loans/overdue` - Overdue loans
  - [ ] `PUT /api/v1/loans/{id}/return` - Return volume
- [ ] Loan business logic
  - [ ] Title-based loan with automatic volume selection
  - [ ] Loan duration by title type (fiction 21d, non-fiction 14d, etc.)
  - [ ] Overdue calculation

### Phase 6: Title-Author Relationships 🔄 **10% COMPLETE**

**Status:** Database junction table ready, implementation needed.

#### Frontend
- [ ] Author assignment in title create/edit
- [ ] Author role selection (main author, co-author, translator, etc.)
- [ ] Display order management
- [ ] Display authors on title list/detail

#### Backend
- [x] title_authors junction table with role and display_order
- [ ] Add author to title endpoint
- [ ] Remove author from title endpoint
- [ ] Update author role/order endpoint
- [ ] Include authors in title responses

### Phase 7: Advanced Features ⏳ **NOT STARTED**

#### Search and Filtering
- [ ] Frontend: Advanced search interface
- [ ] Backend: Search endpoints with filters
  - [ ] Full-text search in titles
  - [ ] Filter by availability, condition, location
  - [ ] Filter by genre, author, series

#### Statistics and Reports
- [ ] Frontend: Dashboard with statistics
  - [ ] Total volumes count
  - [ ] Active loans count
  - [ ] Popular titles (most loaned)
  - [ ] Overdue items
- [ ] Backend: Statistics endpoints
  - [ ] Collection metrics
  - [ ] Loan statistics
  - [ ] Usage analytics

#### Series Management (Not Started)
- [ ] Database: series table
- [ ] Frontend: Series management UI
- [ ] Backend: Series API
- [ ] Series numbering for titles

#### Dewey Classification UI
- [ ] Frontend: Dewey code selector/autocomplete
- [ ] Backend: Dewey code validation
- [ ] Browse by classification

### Phase 8: Import/Export ⏳ **NOT STARTED**

#### Data Import
- [ ] CSV import for bulk title/volume addition
- [ ] ISBN metadata lookup (Google Books API)
- [ ] Duplicate detection during import

#### Data Export
- [ ] Export to CSV
- [ ] Export to JSON
- [ ] Backup/restore functionality

### Phase 9: Polish and Deployment ⏳ **NOT STARTED**

#### User Experience
- [ ] Internationalization (French/English)
- [ ] Keyboard shortcuts
- [ ] Accessibility improvements
- [ ] Error handling and user feedback

#### Quality and Testing
- [ ] Unit tests for business logic
- [ ] Integration tests for API
- [ ] UI testing where applicable
- [ ] Code coverage analysis

#### Deployment
- [ ] Build optimization
- [ ] Release packaging
- [ ] Installation instructions
- [ ] User documentation

## Technical Decisions

### Why Slint?
- **Cross-platform**: Same codebase for native desktop AND web (WASM)
- **Declarative UI**: Clean, maintainable `.slint` files
- **Rust throughout**: Full Rust stack (UI + backend)
- **Native-first development**: Fast compile-test cycles during development
- **WASM ready**: Can compile to WebAssembly when needed
- **No JavaScript**: Pure Rust (no JS dependencies)
- **Type safety**: Shared models between frontend/backend
- **Performance**: Native speed on desktop, near-native in browser

### Why actix-web?
- **Performance**: One of the fastest Rust web frameworks
- **Mature**: Well-tested and production-ready
- **Async**: Built on tokio for efficient I/O
- **Flexible**: Easy to structure with middleware and services

### Why MariaDB?
- **Robust**: Production-grade, enterprise-ready database
- **MySQL-compatible**: Wide ecosystem and tooling support
- **Open-source**: Free and actively maintained
- **Performance**: Optimized for concurrent access and large datasets
- **Features**: Full-text search, JSON support, transactions, foreign keys
- **Scalability**: Can handle growth from personal to multi-user use
- **Backup/Restore**: Mature tools for data safety

## Development Workflow

### Frontend Development (Native - Current)
```bash
cd frontend
cargo run              # Run native application
cargo build --release  # Build optimized native binary
```

### Frontend Development (WASM - Future)
```bash
cd frontend
wasm-pack build --target web --dev    # Build WASM
miniserve . --index index.html -p 8080 # Serve web app
# Open browser to http://localhost:8080
```

### Backend Development
```bash
cd backend
cargo run              # Start API server on :8000
cargo test             # Run tests
cargo clippy           # Lint code
```

### Full Stack Development
```bash
# Terminal 1: Run backend
cd backend && cargo run

# Terminal 2: Build and serve frontend
cd frontend
wasm-pack build --target web --dev
python3 -m http.server 8080

# Open browser to http://localhost:8080
```

## Estimated Timeline

- **Phase 1**: 2-3 weeks (Infrastructure)
- **Phase 2**: 2-3 weeks (Core functionality)
- **Phase 3**: 1 week (Barcode scanning)
- **Phase 4**: 2 weeks (Loan management)
- **Phase 5**: 3-4 weeks (Advanced features)
- **Phase 6**: 1-2 weeks (Import/export)
- **Phase 7**: 1-2 weeks (Polish)

**Total estimated**: 12-17 weeks

*Note: Timeline assumes part-time development (15-20 hours/week)*

## Key Principles

1. **Incremental Development**: Each phase delivers working features
2. **Test as You Go**: Write tests alongside features
3. **User-Centered**: Focus on usability for personal library management
4. **Keep It Simple**: Avoid over-engineering for personal use
5. **Data Integrity**: Strong validation and constraints
6. **Offline-First**: Frontend works without backend when possible

## Success Criteria

- ✅ Can add and manage titles and volumes
- ✅ Can scan barcodes to find books
- ✅ Can loan and return volumes
- ✅ Can track who has which books
- ✅ Can see overdue loans
- ✅ Data is persistent and reliable
- ✅ Interface is intuitive and responsive
- ✅ Works in modern web browsers
- ✅ Accessible from multiple devices

## Critical Path to MVP (Minimum Viable Product)

**Current Status: ~60% Complete**

The following features are CRITICAL for a functional library management system:

### 1. Volume Management (HIGH PRIORITY - BLOCKING)
Without volumes, you cannot track physical books or perform loans. This is the #1 priority.

**Tasks:**
- [ ] Backend: Volume model and database handlers (CRUD operations)
- [ ] Backend: Barcode auto-generation service (VOL-000001 format)
- [ ] Frontend: Volume data models
- [ ] Frontend: Volumes page with list/create/edit
- [ ] Frontend: "Add Volume" button on Titles page
- [ ] UI: Display volumes for each title
- [ ] UI: Location assignment dropdown
- [ ] UI: Condition tracking (excellent/good/fair/poor/damaged)

**Estimated effort:** 2-3 days

### 2. Loan Management (HIGH PRIORITY - BLOCKING)
Core functionality for tracking who borrowed what.

**Tasks:**
- [ ] Backend: Borrower CRUD handlers
- [ ] Backend: Loan CRUD handlers with volume selection logic
- [ ] Backend: Due date calculation by title type
- [ ] Frontend: Borrowers page (simple CRUD)
- [ ] Frontend: Loans page (create loan, return, view active/history)
- [ ] UI: Loan status indicators (available/loaned badges)
- [ ] UI: Overdue warnings

**Estimated effort:** 2-3 days

### 3. Title-Author Relationships (MEDIUM PRIORITY)
Important for proper book metadata.

**Tasks:**
- [ ] Backend: Title-Author junction handlers (add/remove/update)
- [ ] Frontend: Author selection in title create/edit
- [ ] Frontend: Display authors on title list/detail
- [ ] UI: Role selection (main author, co-author, translator, etc.)

**Estimated effort:** 1 day

### 4. Critical Bug Fixes (MEDIUM PRIORITY)
- [ ] Backend: Implement DELETE /api/v1/titles/{id} endpoint
- [ ] Frontend: Add title deletion button and confirmation
- [ ] UI: Error handling and user feedback for failed operations
- [ ] UI: Loading indicators during API calls

**Estimated effort:** 1 day

### 5. Basic Barcode Support (MEDIUM PRIORITY)
At minimum, allow manual barcode entry and lookup.

**Tasks:**
- [ ] Backend: GET /api/v1/scan/volume/{barcode} endpoint
- [ ] UI: Barcode input field for quick lookup
- [ ] UI: Display volume/title details from barcode scan

**Estimated effort:** 0.5 days

### Total Estimated Effort to MVP: ~7-8 days

**After these 5 items, you will have a functional library management system that can:**
- ✅ Add books (titles) with full metadata
- ✅ Add physical copies (volumes) with barcodes and locations
- ✅ Track locations in a hierarchy
- ✅ Manage borrowers
- ✅ Create loans and track returns
- ✅ Associate authors with titles
- ✅ Categorize by genre and publisher

## Future Enhancements (Post-MVP)

- Progressive Web App (PWA) with offline support
- WASM compilation for web deployment
- Native mobile apps (using same backend)
- Advanced barcode scanning with camera/USB scanner
- Advanced reporting and analytics
- Book recommendations
- Integration with online catalogs (Google Books API, Open Library)
- Series management
- Dewey decimal classification UI
- Duplicate detection algorithms
- Import/export (CSV, JSON)
- Search and filtering capabilities
- Statistics dashboard
- Multi-user support with permissions
- Cloud deployment (Docker, Kubernetes)
