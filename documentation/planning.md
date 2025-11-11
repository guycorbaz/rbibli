# Development Planning - Personal Library Manager (rbibli)

## Architecture Overview

The project uses a **web application architecture**:
- **Frontend**: Slint UI framework compiled to WebAssembly
- **Backend**: REST API using actix-web + tokio

This separation allows for:
- Browser-based access from any device
- Independent development of UI and business logic
- Potential future native/mobile apps using the same backend
- Standard web deployment patterns

## Current Status

### Frontend (Slint Web UI - WASM)
- ✅ Basic Slint project structure created
- ✅ Sidebar navigation component
- ✅ Page routing structure (About page implemented)
- ⏳ WASM build configuration
- ⏳ Core UI pages (Volumes, Loans, Scanner, etc.)
- ⏳ Data models and state management
- ⏳ HTTP client for backend API calls
- ⏳ Integration with backend API

### Backend (actix-web API)
- ✅ Basic actix-web project structure
- ✅ Tokio async runtime configured
- ⏳ Database integration
- ⏳ API endpoints implementation
- ⏳ Business logic services
- ⏳ Data repositories

## Development Phases

### Phase 1: Core Infrastructure (In Progress)

#### Frontend Tasks
- [ ] Complete page structure for all main sections
  - [ ] Volumes page
  - [ ] Loans page
  - [ ] Scanner page
  - [ ] Statistics page
- [ ] Define shared data models (Title, Volume, Loan, Borrower)
- [ ] Implement state management with Slint signals
- [ ] Create reusable UI components
  - [ ] Title card component
  - [ ] Volume list component
  - [ ] Loan card component
  - [ ] Search bar component

#### Backend Tasks
- [ ] MariaDB database integration
- [ ] Define database schema
  - [ ] titles table
  - [ ] volumes table
  - [ ] borrowers table
  - [ ] loans table
- [ ] Set up sqlx with MariaDB feature
- [ ] Database migrations (sqlx-cli)
- [ ] Implement repository pattern
- [ ] Create basic CRUD endpoints
- [ ] Health check endpoint

### Phase 2: Title and Volume Management

#### Frontend
- [ ] Title management interface
  - [ ] List view with search/filter
  - [ ] Add new title form
  - [ ] Edit title form
  - [ ] Title detail view
- [ ] Volume management interface
  - [ ] Add volume to title
  - [ ] Edit volume details
  - [ ] Display volume status (available/loaned)
  - [ ] Volume condition tracking

#### Backend
- [ ] Title CRUD API
  - [ ] `GET /api/v1/titles` - List titles
  - [ ] `POST /api/v1/titles` - Create title
  - [ ] `GET /api/v1/titles/{id}` - Get title details
  - [ ] `PUT /api/v1/titles/{id}` - Update title
  - [ ] `DELETE /api/v1/titles/{id}` - Delete title
- [ ] Volume CRUD API
  - [ ] `POST /api/v1/titles/{id}/volumes` - Add volume
  - [ ] `GET /api/v1/volumes/{id}` - Get volume
  - [ ] `PUT /api/v1/volumes/{id}` - Update volume
  - [ ] `DELETE /api/v1/volumes/{id}` - Delete volume
- [ ] Barcode generation service
  - [ ] Sequential barcode generator (VOL-000001)
  - [ ] Uniqueness validation

### Phase 3: Barcode Scanning

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

### Phase 4: Loan Management

#### Frontend
- [ ] Borrower management
  - [ ] Add/edit borrower
  - [ ] List borrowers
  - [ ] Search borrowers
- [ ] Loan operations
  - [ ] Create loan interface
  - [ ] Return volume interface
  - [ ] View active loans
  - [ ] View loan history
- [ ] Loan status indicators
  - [ ] Available/loaned badges
  - [ ] Overdue warnings

#### Backend
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
  - [ ] Loan duration by title type
  - [ ] Overdue calculation

### Phase 5: Advanced Features

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
  - [ ] Popular titles
  - [ ] Overdue items
- [ ] Backend: Statistics endpoints
  - [ ] Collection metrics
  - [ ] Loan statistics
  - [ ] Usage analytics

#### Author and Series Management
- [ ] Frontend: Author/series UI
- [ ] Backend: Author/series API
- [ ] Many-to-many relationships (titles ↔ authors)

#### Dewey Classification
- [ ] Frontend: Dewey code selector
- [ ] Backend: Dewey code validation
- [ ] Browse by classification

### Phase 6: Import/Export

#### Data Import
- [ ] CSV import for bulk title/volume addition
- [ ] ISBN metadata lookup (Google Books API)
- [ ] Duplicate detection during import

#### Data Export
- [ ] Export to CSV
- [ ] Export to JSON
- [ ] Backup/restore functionality

### Phase 7: Polish and Deployment

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

### Why Slint for Web?
- **Declarative UI**: Clean, maintainable `.slint` files
- **Rust throughout**: Full Rust stack (UI + backend)
- **WASM performance**: Near-native speed in browser
- **No JavaScript**: Pure Rust compiled to WebAssembly
- **Type safety**: Shared models between frontend/backend
- **Smaller bundles**: Efficient WASM output

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

### Frontend Development (WASM)
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

## Future Enhancements (Post-MVP)

- Progressive Web App (PWA) with offline support
- Native mobile apps (using same backend)
- Native desktop version (Slint can compile to native)
- Advanced reporting and analytics
- Book recommendations
- Integration with online catalogs (Google Books API)
- Multi-user support with permissions
- Cloud deployment (Docker, Kubernetes)
