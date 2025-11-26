# Development Planning & Status - rbibli

**Last Updated**: 2025-11-26

## Project Overview

**rbibli** is a personal library management system built entirely in Rust using Slint for the user interface. The project aims to manage a personal library with features for tracking books (titles and volumes), loans, authors, series, and includes barcode scanning support.

### Architecture

**Client-Server Architecture**:
- **Frontend**: Slint UI framework (native desktop, WASM support in progress)
- **Backend**: REST API using actix-web + tokio
- **Database**: MariaDB via SQLx

**Development Approach**: Native-first development for faster iteration, with WASM compilation planned for later deployment. This allows:
- Fast compile-test cycles during development
- Native performance and debugging tools
- Easy browser-based access later from any device
- Independent development of UI and business logic
- Standard web deployment patterns

---

## Current Implementation Status

**Overall Progress: ~88% Complete** 🟢

### ✅ Fully Implemented Features

#### Frontend (Slint Native UI)
- ✅ Complete Slint project structure (native desktop)
- ✅ Main application window with ScrollView for responsive content
- ✅ Sidebar navigation component (8 menu items)
- ✅ **7 Fully Functional Pages:**
  - ✅ **Titles Page** - Full CRUD with genre dropdown, volume counts, delete confirmation
  - ✅ **Authors Page** - Full CRUD with biographical information
  - ✅ **Publishers Page** - Full CRUD with company details
  - ✅ **Genres Page** - Full CRUD with title counts
  - ✅ **Locations Page** - Full CRUD with hierarchical structure
  - ✅ **Loans Page** - Complete loan management with tabbed interface (Active Loans, Create Loan, Borrowers, Groups)
  - ✅ **Statistics Page** - Library analytics with visual charts
  - ✅ **About Page** - Application information
- ✅ Modal dialogs for create/edit operations with Save/Cancel buttons
- ✅ Confirmation dialogs for destructive operations
- ✅ Data models (models.rs): Title, Author, Publisher, Genre, Location, Borrower, BorrowerGroup, Loan, Volume, Statistics
- ✅ HTTP API client (api_client.rs) with reqwest
- ✅ Full integration with backend API via callbacks
- ✅ Internationalization infrastructure (`@tr()` macro)
- ✅ **WASM Support**: Async UI refactoring completed for WASM compatibility

#### Backend (actix-web + MariaDB)
- ✅ Complete actix-web server structure with routing
- ✅ Tokio async runtime configured
- ✅ **MariaDB integration** with SQLx (15 migrations applied)
- ✅ **Connection pooling** (MySqlPoolOptions, max 5 connections)
- ✅ **Health check endpoints** (/health, /health/db)
- ✅ **Full CRUD APIs for 11 Entity Types:**
  - ✅ Titles API (GET, POST, PUT, DELETE with business rules)
  - ✅ Volumes API (GET, POST, PUT, DELETE with barcode support)
  - ✅ Authors API (full CRUD)
  - ✅ Publishers API (full CRUD)
  - ✅ Genres API (full CRUD)
  - ✅ Locations API (full CRUD with recursive CTEs)
  - ✅ Borrowers API (full CRUD with group association)
  - ✅ Borrower Groups API (full CRUD with loan policies)
  - ✅ Loans API (create by barcode, list active/overdue, return)
  - ✅ Statistics API (library overview, genres, locations, loans)
  - ✅ ISBN Lookup API (Google Books integration)
  - ✅ Cover Upload API (upload, get, delete)
- ✅ **Dewey Classification**: Simplified manual input (code + category)
- ✅ UUID-based entity IDs (CHAR(36))
- ✅ Timestamp management (created_at, updated_at)
- ✅ Repository pattern for all entities

#### Database Schema (MariaDB)
- ✅ **15 Migrations Applied** - Complete schema:
  - ✅ titles (with publisher_id, genre_id FKs)
  - ✅ volumes (with barcode, condition, loan_status, location_id FK)
  - ✅ authors
  - ✅ publishers
  - ✅ genres
  - ✅ locations (self-referencing hierarchy)
  - ✅ title_authors (junction table with role enum, display_order)
  - ✅ borrowers
  - ✅ borrower_groups (with loan_duration_days policy)
  - ✅ loans (with title_id, volume_id, borrower_id FKs)
- ✅ Foreign key relationships with proper CASCADE/RESTRICT/SET NULL
- ✅ Enum types (condition, loan_status, author_role)
- ✅ Unique constraints (barcodes, etc.)

### 🔄 Partially Implemented

- 🔄 **Title-Author Relationships** (database ready, handlers/UI needed)
  - Database junction table exists with role support
  - Need endpoints to add/remove/update author relationships
  - Need UI for author selection in title forms

### ⏳ Not Yet Implemented

- ⏳ **Series Management** (database schema ready, implementation needed)
- ⏳ **Barcode Generation** (Code 128 format generator)
- ⏳ **Advanced Search & Filtering** (full-text search, complex filters)
- ⏳ **Import/Export** (CSV, JSON formats)
- ⏳ **Duplicate Detection** (fuzzy matching algorithms)
- ⏳ **Loan Extension** (extend loan due dates)

---

## Feature Breakdown by Category

### 1. Title Management (✅ 95% Complete)

**What's Working:**
- ✅ List all titles with volume counts (LEFT JOIN)
- ✅ Create new titles with full metadata
- ✅ Edit existing titles (partial updates supported)
- ✅ Delete titles (only if no volumes exist - business rule enforced)
- ✅ Genre dropdown integration
- ✅ Publisher foreign key relationship
- ✅ ISBN field support
- ✅ Dewey classification fields (manual input)
- ✅ Cover URL field
- ✅ Timestamps (created_at, updated_at)

**Missing:**
- ⏳ Author assignment UI (database ready)
- ⏳ Title detail view page
- ⏳ Search/filter functionality
- ⏳ Cover image upload UI

### 2. Volume Management (✅ 100% Complete)

**What's Working:**
- ✅ List volumes by title
- ✅ Create volumes with auto-generated barcodes (VOL-000001 format)
- ✅ Edit volume details (condition, location, notes)
- ✅ Delete volumes (if not loaned)
- ✅ Automatic copy numbering per title
- ✅ Condition tracking (excellent/good/fair/poor/damaged)
- ✅ Loan status tracking (available/loaned/overdue/lost/maintenance)
- ✅ Location assignment with FK to locations
- ✅ Individual volume notes

**Missing:**
- ⏳ Volume list page in frontend (database and API ready)
- ⏳ Add volume UI from titles page

### 3. Author Management (✅ 90% Complete)

**What's Working:**
- ✅ List all authors with title counts
- ✅ Create new authors with biographical information
- ✅ Edit authors
- ✅ Delete authors (CASCADE to title_authors)
- ✅ Birth/death dates
- ✅ Nationality, biography, website fields

**Missing:**
- ⏳ Title-author relationship management (add/remove authors to titles)
- ⏳ Author role selection (main author, co-author, translator, etc.)
- ⏳ Display order management

### 4. Publisher Management (✅ 100% Complete)

**What's Working:**
- ✅ Full CRUD operations
- ✅ Company details (founded year, country, website)
- ✅ Title count display
- ✅ Used in titles via publisher_id FK

### 5. Genre Management (✅ 100% Complete)

**What's Working:**
- ✅ Full CRUD operations
- ✅ Genre dropdown fully integrated in title forms
- ✅ Title count display for each genre

### 6. Location Management (✅ 100% Complete)

**What's Working:**
- ✅ Full hierarchical structure (parent-child relationships)
- ✅ Recursive CTE for full path display ("Office > Bookshelf A > Shelf 3")
- ✅ Parent location dropdown in create form
- ✅ Volume count per location
- ✅ Used in volumes via location_id FK

### 7. Loan Management (✅ 100% Complete)

**What's Working:**
- ✅ **Borrower Management**:
  - Full CRUD operations with contact information
  - Edit dialog with Save/Cancel buttons
  - Group association for loan policies
- ✅ **Borrower Group Management**:
  - Full CRUD operations
  - Configurable loan duration per group (in days)
  - Edit dialog with Save/Cancel buttons
- ✅ **Loan Operations**:
  - Create loans by scanning/entering volume barcodes
  - Automatic due date calculation based on borrower group policy
  - View all active loans with due dates
  - Visual overdue highlighting
  - Return workflow with volume status update
  - Tabbed interface (Active Loans, Create Loan, Borrowers, Groups)

**Missing:**
- ⏳ Title-based loan with automatic volume selection (currently manual barcode)
- ⏳ Loan extension functionality
- ⏳ Loan history view

### 8. Statistics (✅ 100% Complete)

**What's Working:**
- ✅ **Library Overview**:
  - Total counts (titles, volumes, authors, publishers, genres, locations, borrowers)
  - Active and overdue loans count
  - Visual cards with color coding
- ✅ **Volumes per Genre**:
  - Bar chart visualization
  - Shows both volume and title counts
  - Proportional visual representation
- ✅ **Volumes per Location**:
  - Hierarchical location paths
  - Volume count per location
- ✅ **Loan Status Breakdown**:
  - Count per loan status type

### 9. ISBN Lookup (✅ 100% Complete)

**What's Working:**
- ✅ Google Books API integration
- ✅ Lookup by ISBN (10 or 13 digit)
- ✅ Returns title, authors, publisher, description, cover URL, etc.

### 10. Dewey Classification (✅ 100% Complete - Simplified)

**What's Working:**
- ✅ Manual entry of Dewey Code and Category in Title forms
- ✅ Database fields in `titles` table
- ✅ Display in title lists

**Removed/Simplified:**
- ❌ Complex Dewey Classification database table (removed)
- ❌ Search/Browse Dewey API (removed)
- ❌ Auto-suggestion (removed)

### 11. Cover Images (✅ 100% Complete - Backend)

**What's Working:**
- ✅ Upload cover images (JPEG, PNG, GIF, max 5MB)
- ✅ Get cover image by title ID
- ✅ Delete cover image

**Missing:**
- ⏳ Cover upload UI in frontend
- ⏳ Cover display in title list/detail

---

## Development Phases

### Phase 1: Core Infrastructure ✅ **COMPLETED**

**Status**: 100% Complete

- [x] Database setup (MariaDB + SQLx)
- [x] Complete schema with 15 migrations
- [x] Backend API structure (actix-web + tokio)
- [x] Frontend structure (Slint native)
- [x] API client and communication layer
- [x] Repository pattern for all entities
- [x] Health check endpoints

### Phase 2: Basic Entity Management ✅ **COMPLETED**

**Status**: 100% Complete

- [x] Titles full CRUD
- [x] Authors full CRUD
- [x] Publishers full CRUD
- [x] Genres full CRUD
- [x] Locations full CRUD with hierarchy
- [x] Genre dropdown integration
- [x] Business rule enforcement (title deletion)

### Phase 3: Advanced Features ✅ **COMPLETED**

**Status**: 100% Complete

- [x] Volume management (full CRUD)
- [x] Borrower management (full CRUD)
- [x] Borrower Groups with loan policies
- [x] Loan management (create by barcode, list, return)
- [x] Statistics dashboard (4 endpoint types)
- [x] ISBN lookup integration
- [x] Dewey classification system (Simplified)
- [x] Cover image upload API
- [x] Overdue loan detection and highlighting

### Phase 4: Polish & Integration 🔄 **IN PROGRESS (~60%)**

**Status**: 60% Complete

**Completed**:
- [x] Statistics page with visual charts
- [x] Confirmation dialogs for destructive actions
- [x] Modal dialogs with Save/Cancel buttons
- [x] Borrower/Group edit functionality
- [x] WASM Async UI refactoring

**Remaining**:
- [ ] Title-Author relationship UI
- [ ] Volume list page in frontend
- [ ] Loan extension functionality
- [ ] Error handling and user feedback improvements
- [ ] Loading indicators during API calls
- [ ] Cover image upload UI

### Phase 5: Advanced Features ⏳ **NOT STARTED**

- [ ] Series management
- [ ] Advanced search and filtering
- [ ] Duplicate detection algorithms
- [ ] Import/export (CSV, JSON)
- [ ] Barcode generation (Code 128)
- [ ] Full-text search
- [ ] Loan history view

### Phase 6: Deployment & Polish ⏳ **NOT STARTED**

- [ ] WASM compilation for web deployment
- [ ] Full internationalization (French/English)
- [ ] Comprehensive error handling
- [ ] Unit and integration tests
- [ ] Performance optimization
- [ ] Docker configuration
- [ ] User documentation

---

## Technical Decisions

### Why Slint?
- **Cross-platform**: Same codebase for native desktop AND web (WASM)
- **Declarative UI**: Clean, maintainable `.slint` files
- **Rust throughout**: Full Rust stack (UI + backend)
- **Native-first**: Fast compile-test cycles during development
- **WASM ready**: Can compile to WebAssembly when needed
- **No JavaScript**: Pure Rust (no JS dependencies)
- **Type safety**: Shared models between frontend/backend

### Why actix-web?
- **Performance**: One of the fastest Rust web frameworks
- **Mature**: Well-tested and production-ready
- **Async**: Built on tokio for efficient I/O
- **Flexible**: Easy to structure with middleware and services

### Why MariaDB?
- **Robust**: Production-grade, enterprise-ready database
- **MySQL-compatible**: Wide ecosystem and tooling support
- **Performance**: Optimized for concurrent access
- **Features**: Full-text search, JSON support, transactions, foreign keys
- **Scalability**: Can handle growth from personal to multi-user use
- **Backup/Restore**: Mature tools for data safety

---

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
# Open browser to http://localhost:8000
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

# Terminal 2: Run frontend
cd frontend && cargo run

# Application connects to http://localhost:8000
```

### Database Management
```bash
cd backend
sqlx migrate run       # Apply migrations
sqlx migrate revert    # Revert last migration
sqlx migrate info      # Show migration status
```

---

## Critical Path to MVP

**Current Status: ~88% Complete** 🟢

### Completed Critical Features ✅
1. ✅ **Core Data Management** - All entity CRUD operations working
2. ✅ **Volume Management** - Physical book tracking fully implemented
3. ✅ **Loan Management** - Complete loan workflow with borrowers and groups
4. ✅ **Statistics Dashboard** - Analytics and visualizations
5. ✅ **Database Integration** - All 15 migrations applied, full schema
6. ✅ **Dewey Simplification** - Manual entry implemented, complex system removed

### Remaining Critical Features

#### 1. Title-Author Relationships (HIGH PRIORITY)
**Effort**: 1-2 days

**Tasks**:
- [ ] Backend: Junction table handlers (add/remove/update)
- [ ] Frontend: Author selection in title create/edit
- [ ] Frontend: Display authors on title list/detail
- [ ] UI: Role selection (main author, co-author, translator, etc.)

#### 2. Frontend Volume Integration (MEDIUM PRIORITY)
**Effort**: 1-2 days

**Tasks**:
- [ ] Frontend: Volumes page/tab showing volumes per title
- [ ] Frontend: Add volume button on titles page
- [ ] UI: Volume list component
- [ ] UI: Display volume status badges (available/loaned)

#### 3. Polish & UX Improvements (MEDIUM PRIORITY)
**Effort**: 2-3 days

**Tasks**:
- [ ] Loading indicators during API calls
- [ ] Comprehensive error handling and user feedback
- [ ] Toast notifications for success/error
- [ ] Form validation improvements
- [ ] Empty states for lists
- [ ] Cover image upload UI

### Total Estimated Effort to Feature-Complete: ~5-7 days

---

## Post-MVP Enhancement Roadmap

### Short-term (1-2 weeks)
- Series management (database ready)
- Loan extension functionality
- Cover image upload UI
- Advanced error handling

### Medium-term (1-2 months)
- Advanced search and filtering
- Full-text search implementation
- Duplicate detection algorithms
- Import/export (CSV, JSON)
- Barcode generation (Code 128)
- Loan history view
- Pagination for large lists

### Long-term (2-3 months)
- WASM compilation for web deployment
- Progressive Web App (PWA) features
- Full internationalization (French/English)
- Mobile-responsive design
- Cloud deployment (Docker, Kubernetes)
- Comprehensive testing suite
- User documentation and help system

---

## Success Criteria

### MVP Success Criteria ✅ (Mostly Achieved)
- ✅ Can add and manage titles and volumes
- ✅ Can track physical book locations hierarchically
- ✅ Can manage authors, publishers, and genres
- ✅ Can loan and return volumes with barcode lookup
- ✅ Can track who has which books
- ✅ Can see overdue loans visually highlighted
- ✅ Can view library statistics and analytics
- 🔄 Can associate authors with titles (90% - UI pending)
- ✅ Data is persistent and reliable in MariaDB
- ✅ Interface is intuitive and responsive with native desktop performance

### Full Product Success Criteria (Future)
- ⏳ Can scan barcodes with hardware scanner
- ⏳ Can import books from CSV/JSON
- ⏳ Can export library data
- ⏳ Can detect duplicate entries automatically
- ⏳ Works in modern web browsers via WASM
- ⏳ Accessible from multiple devices
- ⏳ Available in French and English

---

## Timeline Estimates

### Completed Work
- **Phase 1** (Infrastructure): ✅ 3 weeks (Completed)
- **Phase 2** (Basic Entities): ✅ 3 weeks (Completed)
- **Phase 3** (Advanced Features): ✅ 4 weeks (Completed)

### Remaining Work
- **Phase 4** (Polish & Integration): 1-2 weeks
- **Phase 5** (Advanced Features): 3-4 weeks
- **Phase 6** (Deployment): 1-2 weeks

**Total Remaining**: ~5-8 weeks (part-time development)

---

## Key Principles

1. **Data Integrity First**: Strong validation and constraints at database level
2. **Incremental Development**: Each phase delivers working features
3. **User-Centered Design**: Focus on usability for personal library management
4. **Keep It Simple**: Avoid over-engineering for personal/family use
5. **Native Performance**: Leverage Rust and Slint for fast, responsive UI
6. **Type Safety**: Use Rust's type system to prevent bugs
7. **Trust-Based System**: Simple loan management without complex restrictions

---

## Recent Milestones

**November 2024**:
- ✅ Project initialization and architecture decisions
- ✅ Database schema design (13 migrations)
- ✅ Core infrastructure setup

**January 2025**:
- ✅ Full CRUD for 5 core entities (Titles, Authors, Publishers, Genres, Locations)
- ✅ MariaDB integration with SQLx
- ✅ Frontend-backend communication layer

**November 2025** (Recent):
- ✅ Complete loan management system (borrowers, groups, loans)
- ✅ Statistics dashboard with 4 visualization types
- ✅ Volume management full CRUD
- ✅ ISBN lookup via Google Books API
- ✅ **Dewey Decimal Classification Simplification** (Manual input only)
- ✅ **WASM Async UI Refactoring**
- ✅ Cover image upload API
- ✅ Edit dialogs with Save/Cancel pattern
- ✅ Overdue loan detection and highlighting

---

## Project Statistics

**Codebase Size**:
- Frontend: ~2,500 lines of Rust + ~3,000 lines of Slint UI
- Backend: ~3,500 lines of Rust
- Database: 15 SQL migrations

**API Endpoints**: 60+ REST endpoints across 11 entity types

**Database Tables**: 10 tables with comprehensive relationships (Dewey table removed)

**Features**: 8 major feature areas implemented

**Test Coverage**: To be added (planned for Phase 6)

---

## Next Actions (Prioritized)

### This Week
1. Implement title-author relationship UI
2. Add volumes page/tab in frontend
3. Improve error handling and user feedback

### Next Week
4. Add loading indicators
5. Implement cover image upload UI

### This Month
6. Series management implementation
7. Loan extension functionality
8. Advanced search and filtering
9. Finalize WASM compilation setup

**Focus**: Complete Phase 4 (Polish & Integration) to achieve feature-complete status.
