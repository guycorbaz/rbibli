# Missing Features Analysis - rbibli

**Analysis Date**: November 10, 2024

## Current Implementation Status

### ✅ What's Currently Implemented

#### Frontend
- Basic Slint project structure (but **NOT configured for WASM**)
- Main application window with layout
- Sidebar navigation component with 5 menu items
- About page (minimal, shows Slint info only)
- Internationalization macro support (`@tr()`)

#### Backend
- Basic actix-web server structure
- Tokio async runtime configured
- `/health_check` endpoint
- Basic greeting endpoint (development only)

---

## 📝 Development Approach: Native-First, WASM Later

### Current Configuration: Native/Desktop (Intentional)

**Development Strategy**:
- Start with native desktop development for faster iteration and easier debugging
- Slint supports easy cross-compilation to WASM later
- Same `.slint` UI files work for both native and WASM targets

**WASM compilation will be added later** when core features are implemented.

#### WASM Setup (To be added later):
- [ ] Add WASM dependencies (`wasm-bindgen`, `web-sys`, `wasm-bindgen-futures`)
- [ ] Create `index.html` for web deployment
- [ ] Configure WASM build target
- [ ] Update main.rs for WASM entry point (alongside native)
- [ ] Add HTTP client for WASM (`gloo-net` or `reqwest` with WASM features)
- [ ] Web server configuration for serving WASM

**Note**: This is a planned feature, not a blocker. Native development continues normally.

---

## Missing Features by Category

## 1. Title Management (0% Complete)

### 1.1 Data Models - MISSING
- [ ] Title struct/model
- [ ] Fields: title, subtitle, ISBN, publisher, publication_year
- [ ] Fields: pages, language, genre, summary, cover_url
- [ ] Fields: dewey_code, dewey_category
- [ ] Timestamps: created_at, updated_at

### 1.2 UI Pages - MISSING
- [ ] Titles list page (completely missing)
- [ ] Title detail page
- [ ] Add title form
- [ ] Edit title form
- [ ] Title search/filter interface
- [ ] Title card component for display

### 1.3 Backend API - MISSING
- [ ] `GET /api/v1/titles` - List all titles
- [ ] `POST /api/v1/titles` - Create title
- [ ] `GET /api/v1/titles/{id}` - Get title details
- [ ] `PUT /api/v1/titles/{id}` - Update title
- [ ] `DELETE /api/v1/titles/{id}` - Delete title
- [ ] `GET /api/v1/titles/wishlist` - Wishlist (0 volumes)

### 1.4 Features - MISSING
- [ ] Title validation (ISBN format, required fields)
- [ ] Cover image upload/display
- [ ] Authors association
- [ ] Series association
- [ ] Genre/classification management

---

## 2. Volume Management (0% Complete)

### 2.1 Data Models - MISSING
- [ ] Volume struct/model
- [ ] Fields: title_id, copy_number, barcode
- [ ] Fields: condition (excellent/good/fair/poor/damaged)
- [ ] Fields: location, loan_status
- [ ] Fields: individual_notes
- [ ] Timestamps: created_at, updated_at

### 2.2 UI Pages - MISSING
- [ ] Volumes list page (commented out)
- [ ] Volume detail view
- [ ] Add volume form
- [ ] Edit volume form
- [ ] Volume card component
- [ ] Volume status indicators (available/loaned/overdue)
- [ ] Condition selector

### 2.3 Backend API - MISSING
- [ ] `POST /api/v1/titles/{id}/volumes` - Add volume to title
- [ ] `GET /api/v1/volumes/{id}` - Get volume details
- [ ] `PUT /api/v1/volumes/{id}` - Update volume
- [ ] `DELETE /api/v1/volumes/{id}` - Delete volume
- [ ] `GET /api/v1/volumes?title_id={id}` - List volumes by title

### 2.4 Features - MISSING
- [ ] Barcode generation (VOL-000001 format, Code 128)
- [ ] Barcode uniqueness validation
- [ ] Copy numbering (automatic sequential)
- [ ] Physical condition tracking
- [ ] Location management
- [ ] Loan status tracking

---

## 3. Barcode Scanning (0% Complete)

### 3.1 UI - MISSING
- [ ] Scanner page (completely missing)
- [ ] Barcode input field (supports hardware scanners)
- [ ] Manual barcode entry
- [ ] Scan result display
- [ ] Quick actions (loan/return from scan)

### 3.2 Backend API - MISSING
- [ ] `GET /api/v1/scan/volume/{barcode}` - Find by volume barcode
- [ ] `GET /api/v1/scan/isbn/{isbn}` - Find by ISBN
- [ ] `POST /api/v1/scan/loan` - Quick loan via scan
- [ ] `POST /api/v1/scan/return` - Quick return via scan

### 3.3 Features - MISSING
- [ ] Dual barcode support (Volume Code 128 + ISBN EAN-13)
- [ ] Barcode validation (Code 128 format)
- [ ] ISBN validation (EAN-13 checksum)
- [ ] Scanner device integration
- [ ] Scan history

---

## 4. Loan Management (0% Complete)

### 4.1 Data Models - MISSING
- [ ] Loan struct/model
- [ ] Borrower struct/model
- [ ] Fields: title_id, volume_id, borrower_id
- [ ] Fields: loan_date, due_date, return_date
- [ ] Fields: status (active/returned/overdue)

### 4.2 UI Pages - MISSING
- [ ] Loans list page (commented out)
- [ ] Active loans view
- [ ] Overdue loans view
- [ ] Loan history
- [ ] Create loan form
- [ ] Return volume interface
- [ ] Loan detail view
- [ ] Loan card component

### 4.3 Borrower Management - MISSING
- [ ] Subscribers/Borrowers page (commented out)
- [ ] Add borrower form
- [ ] Edit borrower form
- [ ] Borrowers list
- [ ] Borrower search
- [ ] Borrower detail page

### 4.4 Backend API - MISSING
- [ ] `POST /api/v1/loans` - Create loan
- [ ] `GET /api/v1/loans` - List all loans
- [ ] `GET /api/v1/loans/active` - Active loans
- [ ] `GET /api/v1/loans/overdue` - Overdue loans
- [ ] `PUT /api/v1/loans/{id}/return` - Return volume
- [ ] `PUT /api/v1/loans/{id}/extend` - Extend loan
- [ ] `GET /api/v1/borrowers` - List borrowers
- [ ] `POST /api/v1/borrowers` - Create borrower
- [ ] `GET /api/v1/borrowers/{id}` - Get borrower
- [ ] `PUT /api/v1/borrowers/{id}` - Update borrower
- [ ] `DELETE /api/v1/borrowers/{id}` - Delete borrower

### 4.5 Features - MISSING
- [ ] Title-based loan with automatic volume selection
- [ ] Volume selection priority (condition → location → FIFO)
- [ ] Loan duration by title type (Fiction: 21d, Non-fiction: 14d, etc.)
- [ ] Overdue calculation
- [ ] Loan validation (volume available, borrower exists)
- [ ] Return validation

---

## 5. Author & Series Management (0% Complete)

### 5.1 Data Models - MISSING
- [ ] Author struct/model
- [ ] Series struct/model
- [ ] Title-Author relationship (many-to-many)
- [ ] Title-Series relationship

### 5.2 UI - MISSING
- [ ] Authors list page
- [ ] Add/edit author form
- [ ] Author detail page (with their titles)
- [ ] Series list page
- [ ] Add/edit series form
- [ ] Series detail page (with titles in order)
- [ ] Author selector component
- [ ] Series selector component

### 5.3 Backend API - MISSING
- [ ] Author CRUD endpoints
- [ ] Series CRUD endpoints
- [ ] Title-author association endpoints
- [ ] Title-series association endpoints

---

## 6. Search & Filtering (0% Complete)

### 6.1 UI - MISSING
- [ ] Global search bar
- [ ] Advanced search interface
- [ ] Filter by availability
- [ ] Filter by condition
- [ ] Filter by location
- [ ] Filter by genre
- [ ] Filter by author
- [ ] Filter by series
- [ ] Sort options (title, author, date, etc.)

### 6.2 Backend API - MISSING
- [ ] `GET /api/v1/search/titles?q={query}` - Text search
- [ ] `GET /api/v1/search/volumes?filters=...` - Volume search
- [ ] `GET /api/v1/search/authors?q={query}` - Author search
- [ ] Full-text search implementation
- [ ] Filter query parameters

---

## 7. Statistics & Dashboard (0% Complete)

### 7.1 UI - MISSING
- [ ] Statistics page (commented out)
- [ ] Dashboard with overview
- [ ] Total volumes count
- [ ] Active loans count
- [ ] Overdue items count
- [ ] Popular titles chart
- [ ] Collection metrics
- [ ] Loan statistics charts

### 7.2 Backend API - MISSING
- [ ] `GET /api/v1/stats/overview` - Dashboard data
- [ ] `GET /api/v1/stats/loans` - Loan statistics
- [ ] `GET /api/v1/stats/collection` - Collection metrics

---

## 8. Dewey Classification (0% Complete)

### 8.1 Features - MISSING
- [ ] Dewey code data model
- [ ] Dewey code validation
- [ ] Dewey category lookup
- [ ] Dewey code selector UI
- [ ] Browse by classification
- [ ] Reference table for Dewey codes

---

## 9. Duplicate Detection (0% Complete)

### 9.1 Features - MISSING
- [ ] Duplicate candidate model
- [ ] ISBN matching (identical detection)
- [ ] Title + Author fuzzy matching (Levenshtein)
- [ ] Confidence scoring (0.0-1.0)
- [ ] Duplicate detection UI
- [ ] Merge workflow
- [ ] Real-time detection during title creation

---

## 10. Import/Export (0% Complete)

### 10.1 Features - MISSING
- [ ] CSV import
- [ ] CSV export
- [ ] JSON import
- [ ] JSON export
- [ ] ISBN metadata lookup (Google Books API)
- [ ] Bulk import interface
- [ ] Import validation
- [ ] Export options (all data, titles only, etc.)

---

## 11. Internationalization (0% Complete)

### 11.1 Features - MISSING
- [ ] Language files (French/English)
- [ ] Language switcher UI
- [ ] All UI strings translated
- [ ] Date formatting (locale-aware)
- [ ] Number formatting

---

## 12. Database Layer (0% Complete)

### 12.1 Infrastructure - MISSING
- [ ] MariaDB database setup and connection
- [ ] Database migrations (sqlx-cli with MariaDB)
- [ ] Schema creation (SQL scripts)
- [ ] Repository pattern implementation
- [ ] Database abstraction traits
- [ ] Connection pooling (sqlx::Pool)
- [ ] Transaction support
- [ ] Database configuration (connection string, etc.)

### 12.2 Tables - MISSING
- [ ] titles table
- [ ] volumes table
- [ ] authors table
- [ ] series table
- [ ] title_authors junction table
- [ ] borrowers table
- [ ] loans table
- [ ] duplicate_candidates table

---

## 13. API Client & Communication (0% Complete)

### 13.1 Frontend HTTP Client - MISSING
- [ ] HTTP client setup (gloo-net or reqwest-wasm)
- [ ] API client module
- [ ] Request/response serialization
- [ ] Error handling
- [ ] Loading states
- [ ] API base URL configuration
- [ ] CORS handling

---

## 14. State Management (0% Complete)

### 14.1 Features - MISSING
- [ ] Shared state between components
- [ ] Reactive data binding
- [ ] State updates from API responses
- [ ] Loading indicators
- [ ] Error state management
- [ ] Form state management

---

## 15. Authentication & Security (0% Complete)

### 15.1 Features - MISSING
- [ ] Optional login/password
- [ ] Session management
- [ ] Authentication UI
- [ ] CORS configuration
- [ ] Input validation
- [ ] XSS prevention
- [ ] SQL injection prevention

---

## 16. User Experience Features (0% Complete)

### 16.1 Missing UX - MISSING
- [ ] Keyboard shortcuts
- [ ] Confirmation dialogs
- [ ] Toast notifications
- [ ] Error messages
- [ ] Success messages
- [ ] Loading spinners
- [ ] Empty states
- [ ] Pagination
- [ ] Responsive design (mobile/tablet)
- [ ] Accessibility (ARIA labels, keyboard navigation)

---

## 17. Deployment & Build (Partially Missing)

### 17.1 Web Deployment - MISSING
- [ ] Production WASM build configuration
- [ ] Bundle size optimization
- [ ] Docker configuration for web app
- [ ] Nginx configuration
- [ ] Environment variables
- [ ] Production API URL configuration

---

## Summary Statistics

### Implementation Progress:
- **Frontend UI**: ~5% (only basic structure + 1 page)
- **Backend API**: ~2% (only health check)
- **Database**: 0%
- **WASM Configuration**: Deferred (intentional - native-first approach)
- **Data Models**: 0%
- **Business Logic**: 0%
- **Integration**: 0%

### Overall Progress: **~2%** 🔴

### Critical Path Items (Must Do First):
1. **Database Integration** - MariaDB setup, create schema, migrations
2. **Data Models** - Define Title, Volume, Loan, Borrower structs (Rust)
3. **Basic API** - Implement CRUD endpoints for titles and volumes
4. **Core UI Pages** - Volumes page, Titles page, basic forms (Slint)
5. **API Client** - HTTP client in frontend to call backend (native first)
6. **WASM Compilation** - Add WASM build target (later, when features are working)

### Estimated Work Remaining:
Based on planning document estimate of 12-17 weeks, and current ~2% completion:
- **Remaining**: ~12-16 weeks of development

### Next Steps (Recommended Order):
1. **Set up MariaDB database** + create initial schema + migrations (sqlx)
2. **Implement Title data model** + repository pattern + basic CRUD API
3. **Create Titles list page** in Slint with native HTTP client for API calls
4. **Implement Volume management** (following same pattern as Title)
5. **Add barcode generation** service in backend
6. **Implement Loan management** (borrowers + loans)
7. **Add barcode scanning** page and API endpoints
8. **Build remaining features** (search, statistics, etc.)
9. **Add WASM compilation target** (same codebase, different build target)
10. **Deploy** as web application

**Development Flow**: Build features natively → Test/debug quickly → Add WASM target when stable
