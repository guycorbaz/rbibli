# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**rbibli** is a personal library management system built entirely in Rust using **Slint** for the user interface. The project aims to manage a personal library with features for tracking books (titles and volumes), loans, authors, series, and includes barcode scanning support. The application will be deployable as both a **native desktop application** and a **web application** (via WebAssembly), using the same codebase with a REST API backend.

**Development Approach**: Currently developed as a native application for faster iteration, with WASM compilation to be added later. Slint's cross-compilation capabilities allow the same UI code to work for both native and web targets.

**Key Architectural Concept**: The system distinguishes between **Titles** (abstract book metadata) and **Volumes** (physical copies). A title can have 0 to N volumes, allowing for wishlist functionality, duplicate copies tracking, and sophisticated loan management.

## Repository Structure

```
rbibli/
├── frontend/                # Slint UI application
│   ├── src/                 # Rust source code
│   │   └── main.rs         # Main entry point (native, WASM later)
│   ├── ui/                  # Slint UI files (.slint)
│   │   ├── app-window.slint # Main application window
│   │   ├── side_bar.slint   # Navigation sidebar
│   │   ├── pages/           # UI page components
│   │   └── gallery_settings.slint
│   ├── lang/                # Internationalization files (planned)
│   ├── build.rs            # Slint build script
│   ├── Cargo.toml          # Frontend dependencies (Slint 1.14.1)
│   └── .cargo/config.toml  # Build configuration
├── backend/                 # REST API backend (actix-web, tokio)
│   ├── src/                 # API implementation
│   │   ├── main.rs         # API server entry point
│   │   └── lib.rs          # Library code
│   ├── tests/              # API tests
│   └── Cargo.toml          # Backend dependencies
└── documentation/           # Project documentation
    ├── requirements.md      # Complete functional specifications
    ├── architecture.md      # Technical architecture
    ├── api.md              # API documentation
    ├── planning.md          # Development roadmap
    ├── development_environment.md # Development setup
    └── sqlx_installation.md # SQLx CLI installation guide
```

**Note**: WASM-specific files (index.html, wasm-bindgen dependencies) will be added when WASM compilation is configured.

## Current Implementation Status (Phase 2-3)

### ✅ Fully Implemented Features

**Database & Infrastructure:**
- MariaDB integration with SQLx (13 migrations applied)
- Connection pooling (MySqlPoolOptions, max 5 connections)
- Health check endpoints (/health, /health/db)
- UUID-based entity IDs (CHAR(36) format)
- Timestamp management (created_at, updated_at)

**Frontend Pages (Slint UI):**
- **Titles Page**: Create, edit, list titles with genre dropdown and volume counts
- **Authors Page**: Full CRUD operations with biographical information
- **Publishers Page**: Full CRUD operations with company details
- **Genres Page**: Full CRUD operations with title counts
- **Locations Page**: Full CRUD with hierarchical structure and full path display
- **About Page**: Application information

**Backend API Endpoints:**
- GET/POST/PUT /api/v1/titles (DELETE missing)
- GET/POST/PUT/DELETE /api/v1/authors
- GET/POST/PUT/DELETE /api/v1/publishers
- GET/POST/PUT/DELETE /api/v1/genres
- GET/POST/PUT/DELETE /api/v1/locations

**UI Features:**
- Sidebar navigation with 8 menu items
- ScrollView for responsive content areas
- Modal dialogs for create/edit operations
- Data binding between Rust and Slint
- Callback system for API communication
- Genre dropdown in title forms
- Parent location dropdown for hierarchical locations
- Internationalization infrastructure (@tr() macro)

### 🔄 Partially Implemented (Database Schema Ready)

**Volumes:** Database table created with all fields (barcode, condition, location_id, loan_status), but no backend handlers or UI
**Title-Authors:** Junction table ready with role support, but no handlers or UI
**Loans:** Complete database schema with status tracking, but no implementation
**Borrowers:** Database table ready, but no implementation

### ⏳ Not Yet Implemented

- Volume management (CRITICAL for MVP)
- Loan workflow and borrower management
- Barcode generation and scanning
- Title deletion endpoint
- Search and filter capabilities
- Import/export functionality
- Duplicate detection
- Statistics dashboard
- Cover image upload
- Dewey classification UI
- Series management

### 📊 MVP Completion: ~60%

Core infrastructure is excellent, but critical features (Volumes, Loans) are pending.

## Core Development Commands

### Building and Running the Frontend

**Current**: Native application (faster development)

```bash
# Navigate to frontend directory
cd frontend

# Build the project
cargo build

# Run the application
cargo run

# Run in release mode
cargo run --release
```

**Later**: WASM compilation (when ready for web deployment)

```bash
# Install wasm-pack (when needed)
cargo install wasm-pack

# Build for web (development)
wasm-pack build --target web --dev

# Build for web (production)
wasm-pack build --target web --release

# Serve with a web server
python3 -m http.server 8080
# or
cargo install miniserve
miniserve . --index index.html -p 8080
```

### Building and Running the Backend (API)

```bash
# Navigate to backend directory
cd backend

# Build the backend
cargo build

# Run the API server
cargo run

# Run tests
cargo test

# Run release version
cargo run --release
```

### Building the Entire Workspace

```bash
# From the root directory, build all workspace members
cargo build --workspace

# Run tests for all workspace members
cargo test --workspace
```

### Code Quality

```bash
# Run linting
cargo clippy

# Format code
cargo fmt

# Run tests (when implemented)
cargo test

# Run a specific test
cargo test test_name
```

### Development Environment Setup

The project requires:
- **Rust**: Latest stable toolchain (currently using Rust 1.91.0, Cargo 1.91.0)
- **Slint**: UI framework v1.14.1
  - No additional installation needed - included as a Cargo dependency
  - UI files use `.slint` extension with declarative syntax
  - Compiled at build time via `build.rs`
  - Cross-compiles to both native and WASM from same codebase

**Current Development** (Native):
- Standard Rust toolchain
- Native debugging tools
- Fast compile-test cycles

**Future** (WASM - when configured):
- `wasm-pack` for building WASM packages
- `wasm-bindgen` for JavaScript interop
- Web server for serving WASM application

Architecture:
- **Frontend**: Slint UI (currently native, WASM later)
- **Backend**: actix-web REST API for data operations
- **Communication**: Frontend calls backend API via HTTP
- **Database**: MariaDB (integrated with backend)

### Slint UI Framework

The application uses **Slint** for cross-platform UI:
- **UI files**: Located in `ui/*.slint` using declarative Slint language
- **Build integration**: Slint files are compiled at build time via `build.rs`
- **Main window**: `AppWindow` component (defined in `ui/app-window.slint`)
- **Internationalization**: Built-in support via `@tr()` macro for translatable strings
- **Components**: Modular UI components in `ui/pages/` directory
- **Styling**: Slint's built-in styling system
- **Performance**: Native performance (currently), near-native via WASM (later)
- **Cross-compilation**: Same codebase compiles to:
  - Native desktop (Windows, Linux, macOS) - **current**
  - WebAssembly for browsers - **future**
  - Native mobile (potential) - **future**

## Architecture and Data Model

### Title/Volume Hierarchy (Critical Design Pattern)

The core architectural pattern separates **Titles** from **Volumes**:

1. **Title**: Abstract book metadata (ISBN, authors, genre, Dewey classification, cover, summary)
   - One title can have 0 to N volumes
   - Titles with 0 volumes represent the wishlist
   - Classification (Dewey, genre) applies at title level

2. **Volume**: Physical copy with unique properties
   - Each volume belongs to exactly one title
   - Has unique barcode (VOL-000001 format, Code 128)
   - Tracks: copy number, physical condition, location, loan status, individual notes

### Key Entities

- **Title**: Book metadata shared across all copies
- **Volume**: Physical copy with barcode, condition, location, loan status
- **Author**: Writers with roles (main author, co-author, illustrator, translator)
- **Series**: Groups of related titles with numbering
- **Loan**: Tracks volume loans to borrowers (title-based request, volume-specific fulfillment)
- **Borrower**: Simple contact info (name, email, phone) - trust-based system
- **Wishlist**: Simple reservation system instead of complex queues
- **DuplicateCandidate**: Tracks potential duplicate titles for merging

### Barcode System (Dual Strategy)

1. **Volume Barcodes** (Code 128): `VOL-000001` format
   - Unique identifier for each physical copy
   - Used for: loan/return operations, inventory, location tracking

2. **ISBN Barcodes** (EAN-13): Standard 13-digit ISBN
   - Used for: title identification, metadata retrieval, adding new books
   - Supports duplicate detection and external API lookups

### Database Strategy

**MariaDB** as the chosen database:
- **MariaDB**: Production-grade, MySQL-compatible relational database
- **Full feature support**: UUID (via BINARY(16) or CHAR(36)), JSON, full-text search
- **Repository pattern**: Abstract database operations behind traits for clean architecture
- **SQLx**: Compile-time checked queries with async support, connection pooling

### Loan Management Rules

- **Title-based loans**: Users request titles, system selects best available volume
- **Volume selection priority**: Best condition → Most accessible location → Lowest copy number (FIFO)
- **Loan duration by title type**: Fiction (21 days), Non-fiction (14 days), Reference (7 days), Magazine (3 days)
- **Simple workflow**: No complex restrictions, penalties, or suspensions (trust-based for friends/family)

### Duplicate Management

Important system feature for data integrity:
- **Detection types**: Identical ISBN, Title+Author match, Fuzzy match (Levenshtein)
- **Confidence scoring**: 0.0-1.0 scale for match likelihood
- **Merge workflow**: Combines metadata, moves all volumes to primary title, renumbers copies
- **Prevention**: Real-time detection during title creation and import

## Important Implementation Details

### Architecture

**Current Implementation**:
- **Native desktop application** with Slint UI (in `frontend/`)
- **REST API backend** using actix-web (in `backend/`)
- **Client-server architecture** with HTTP for communication
- **Development approach**: Native-first for faster iteration

**Deployment Options** (Same Codebase):
1. **Native Desktop** (current focus):
   - Windows, Linux, macOS executables
   - Direct HTTP to backend API
   - Fast startup, native performance

2. **Web Application** (WASM - to be configured):
   - Browser-based access
   - Slint compiled to WebAssembly
   - Same `.slint` UI files, different build target
   - Accessible from any device

**Implemented Features** (Phase 2-3):
- ✅ Frontend connected to backend REST API
- ✅ MariaDB database fully integrated with SQLx
- ✅ Full CRUD for: Titles, Authors, Publishers, Genres, Locations
- ✅ Hierarchical location management with recursive CTEs
- ✅ Genre dropdown integration in title forms
- ✅ ScrollView for responsive content areas
- ✅ Health check endpoints for monitoring

**In Progress** (Phase 2-3):
- 🔄 Volume management (database schema ready, handlers needed)
- 🔄 Title deletion endpoint (currently missing)
- 🔄 Title-Author relationship management (junction table ready)

**Planned Features** (Phase 3-4):
- ⏳ Loan management system (borrowers, loans, returns)
- ⏳ Barcode generation and scanning (Code 128 format)
- ⏳ User authentication and sessions
- ⏳ Import/export functionality (CSV, JSON)
- ⏳ Duplicate detection algorithms
- ⏳ Statistics and reporting dashboard
- ⏳ WASM build target for web deployment
- ⏳ Progressive Web App (PWA) features
- ⏳ Dewey classification UI
- ⏳ Cover image upload and display

### Key Business Rules

1. **Titles can exist without volumes** (wishlist functionality)
2. **Cannot delete title if any volume is currently loaned**
3. **Volume barcodes must be globally unique** (VOL-XXXXXX format)
4. **Dewey classification applies at title level**, inherited by volumes
5. **Loan status must match volume availability** (referential integrity)
6. **Series contain titles, which contain volumes** (three-level hierarchy)

### Code Organization Pattern

```
frontend/
├── src/
│   ├── main.rs          # Native/WASM entry point, Slint initialization
│   ├── models.rs        # Data structures (Title, Author, Publisher, Genre, Location) ✅
│   ├── api_client.rs    # HTTP client for backend REST API calls ✅
│   └── utils/           # Helpers, validation (planned)
├── ui/
│   ├── app-window.slint    # Main application window
│   ├── side_bar.slint      # Navigation sidebar component
│   ├── pages/              # Page components
│   │   ├── pages.slint     # Page exports
│   │   ├── about_page.slint     # ✅ Implemented
│   │   ├── titles_page.slint    # ✅ Implemented (full CRUD except delete)
│   │   ├── authors_page.slint   # ✅ Implemented (full CRUD)
│   │   ├── publishers_page.slint # ✅ Implemented (full CRUD)
│   │   ├── genres_page.slint    # ✅ Implemented (full CRUD)
│   │   ├── locations_page.slint # ✅ Implemented (full CRUD)
│   │   └── page.slint           # Base page component
│   └── gallery_settings.slint
├── lang/                # Translation files (planned)
├── index.html           # HTML entry point
├── build.rs             # Slint build script
└── Cargo.toml           # Dependencies including wasm-bindgen
```

### Slint UI Architecture

- **Declarative components**: UI defined in `.slint` files with properties, callbacks, and bindings
- **Component hierarchy**: Reusable components composed together
- **Data binding**: Two-way binding between Rust logic and UI properties
- **Callbacks**: Event handling from UI to Rust code via defined callbacks
- **Signals**: Reactive property updates automatically propagate to UI
- **Native widgets**: Platform-native controls and styling

## Development Guidelines

### When Working with Slint UI

**Slint File Structure**:
- Use declarative syntax: `component ComponentName inherits BaseType { ... }`
- Define properties: `in property <type> name;` (input) or `out property <type> name;` (output)
- Create callbacks: `callback name <=> internal.callback;` or `callback name(Type) -> ReturnType;`
- Use bindings: `property: expression;` for reactive updates
- Conditional rendering: `if (condition) : Component {}`

**Translation Strings**:
- Simple: `@tr("key")`
- With context: `@tr("Menu" => "Volumes")`
- Language files will be in `lang/` directory

**Current Navigation**:
- Main navigation items: Volumes, Subscribers, Loans, Statistics, About
- Most pages are placeholder/commented out (`if(side-bar.current-item == 5) : AboutPage {}`)
- About page is currently the only implemented page

**Connecting Rust to Slint**:
```rust
slint::include_modules!();  // Includes generated Rust code from .slint files

fn main() {
    let ui = AppWindow::new()?;  // Create instance of main window

    // Set properties from Rust
    ui.set_property_name(value);

    // Connect callbacks
    ui.on_callback_name(|| {
        // Handle event
    });

    ui.run()?;  // Start event loop
}
```

### When Working with Data Models

1. Always maintain Title/Volume separation
2. Ensure barcode uniqueness validation
3. Apply Dewey classification at title level
4. Track loan status at volume level
5. Use UUIDs for all entity IDs (planned)
6. Include `created_at` and `updated_at` timestamps

### When Implementing Duplicate Detection

- Check ISBN first (absolute priority)
- Use Levenshtein distance for fuzzy title matching
- Normalize strings (lowercase, remove accents, trim whitespace)
- Provide confidence scores for manual review
- Never auto-merge without user confirmation (except identical ISBN)

### Security Considerations

- Simple authentication for personal use (optional login/password)
- Input validation to prevent XSS and SQL injection
- File upload validation (type, size limits)
- Trust-based system (no complex access control needed)
- HTTPS support for remote access (optional)

## Common Tasks

### Adding a New UI Page (Slint)

1. **Create page component**: Add new `.slint` file in `ui/pages/`
   ```slint
   import { Page } from "page.slint";

   export component MyNewPage inherits Page {
       title: @tr("My New Page");
       description: @tr("Description of what this page does");

       // Add your UI elements here
   }
   ```

2. **Export component**: Add export to `ui/pages/pages.slint`
   ```slint
   export { MyNewPage } from "my_new_page.slint";
   ```

3. **Import in main window**: Update `ui/app-window.slint`
   ```slint
   import {AboutPage, MyNewPage} from "pages/pages.slint";
   ```

4. **Add conditional display**: In app-window.slint's layout
   ```slint
   if(side-bar.current-item == 6) : MyNewPage {}
   ```

5. **Add menu item**: Update sidebar model in app-window.slint
   ```slint
   model: [@tr("Menu" => "Volumes"), ..., @tr("Menu" => "My New Page")]
   ```

### Adding a New Entity (When Database is Integrated)

1. **Define struct** in `frontend/src/models/` with Serialize/Deserialize
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct MyEntity {
       pub id: Uuid,
       pub name: String,
       // ... other fields
   }
   ```

2. **Create repository trait** in `frontend/src/repositories/` for database operations
3. **Implement business logic** in `frontend/src/services/`
4. **Connect to Slint UI**: Expose data and operations via callback handlers in `main.rs`
5. **Update UI**: Create/update Slint components to display and interact with entity

### Integrating Rust Logic with Slint UI

**Passing data to UI**:
```rust
let ui = AppWindow::new()?;

// Set simple properties
ui.set_title("My Library".into());

// Set complex data (use Slint's data structures)
let items = Rc::new(VecModel::from(vec![...]));
ui.set_items(items.into());
```

**Handling UI callbacks**:
```rust
ui.on_item_clicked({
    let ui_weak = ui.as_weak();
    move |item_id| {
        let ui = ui_weak.unwrap();
        // Process the click
        // Update UI state
    }
});
```

## External Dependencies

### Frontend Dependencies (Current - Native)
- `slint = "1.14.1"` - Cross-platform UI framework
- `slint-build = "1.14.1"` - Build-time UI compilation

### Frontend Dependencies (To Add - WASM)
When WASM compilation is configured:
- `wasm-bindgen = "*"` - JavaScript/WASM interop
- `web-sys = "*"` - Web APIs for WASM
- `wasm-bindgen-futures = "*"` - Async support in WASM
- `gloo-net = "*"` or `reqwest` (WASM features) - HTTP client for API calls
- HTTP client for backend API calls (to be added)

### Backend Dependencies (Current)
- `actix-web = "4.11.0"` - Web framework for REST API
- `tokio = "1.47.1"` - Async runtime
- `reqwest = "0.11.27"` - HTTP client (dev dependency)

### Planned Dependencies
- **Database**: `sqlx` with MariaDB/MySQL feature for data persistence
- **Serialization**: `serde`, `serde_json` for data handling (workspace dependency)
- **UUID**: `uuid` for entity identifiers
- **Date/Time**: `chrono` for date handling
- **Validation**: `validator` for input validation
- **Barcode**: Barcode generation/validation library
- **CSV/JSON Import**: `csv`, `serde_json` for data import/export

## Testing Strategy (Planned)

- **Unit tests**: Business logic in services (`cargo test`)
- **Integration tests**: Repository layer with test database
- **UI testing**: Slint provides testing capabilities for component logic
- **Property testing**: Use `proptest` for validating business rules
- **Security**: Regular `cargo audit` for dependency vulnerabilities

## Documentation Resources

The `documentation/` folder contains detailed specifications:

- **requirements.md**: Complete functional requirements (767 lines)
  - Title/volume relationship, barcode system, loan rules
  - Scanner interface, search capabilities, import/export
  - Dewey classification, duplicate management, validation rules
  - UI workflows and user interactions

- **architecture.md**: Technical architecture document (2333 lines)
  - Full data models with Rust structs and SQL schemas
  - Service layer patterns and business logic
  - Repository patterns for database abstraction
  - **Note**: Contains some Leptos/web examples that are now outdated
  - Focus on the data models and business logic sections, which remain valid

- **development_environment.md**: Development setup instructions (394 lines)
  - Rust toolchain, WebAssembly tools, Slint setup
  - MariaDB installation and configuration
  - Build commands, testing, code quality tools
  - Platform-specific notes (Windows, Linux, macOS)

- **api.md**: REST API documentation (221 lines)
  - Complete endpoint specifications for titles, volumes, loans, borrowers
  - Data models (JSON schemas)
  - Error handling patterns
  - Database integration approach

- **planning.md**: Development roadmap (304 lines)
  - 7 development phases from infrastructure to deployment
  - Technology choices and rationale
  - MariaDB selection reasoning
  - Native-first development approach

- **sqlx_installation.md**: SQLx CLI installation guide (comprehensive)
  - Platform-specific installation instructions
  - Troubleshooting common issues
  - Migration workflow examples
  - Quick reference commands

**Important**: The architecture document was written when the project was planned as a web application with Leptos. The data models, business rules, and core architectural concepts (Title/Volume separation, duplicate management, etc.) are still valid. The UI implementation details should refer to Slint patterns instead.

## Important Notes

- This is a **personal library manager** designed for small-scale use (friends, family)
- System prioritizes **simplicity and trust** over complex access control
- **Title/Volume distinction** is fundamental to the entire architecture
- **Flexible deployment** using Slint: native desktop (current) or web via WASM (later)
- Architecture changed from Leptos (web framework) to Slint (cross-platform UI framework)
- **Development approach**: Native-first for speed, add WASM compilation later
- Focus on **data integrity** through validation and duplicate detection
- Support for **multiple physical copies** is a core feature, not an afterthought
- **Same codebase** works for both desktop and web deployments

## Technology Decision

The project **transitioned from Leptos to Slint**:

**Why Slint**:
- **Declarative UI**: Clean, maintainable UI definition in `.slint` files
- **Rust throughout**: Full Rust stack from UI to API (type safety, shared models)
- **Cross-compilation**: Same code compiles to native AND WASM
- **No JavaScript**: Pure Rust for all logic
- **Component-based**: Reusable UI components with clear separation
- **Performance**: Native speed (desktop) or near-native (WASM)
- **Flexible deployment**: Choose native or web based on needs

**Development Approach**:
- **Native-first**: Develop as desktop app for faster iteration
  - Faster compile times (no WASM overhead)
  - Easier debugging (native tools)
  - Immediate testing
- **WASM later**: Add web deployment when features are stable
  - Same `.slint` files, different build target
  - No code changes needed for cross-compilation
  - `wasm-pack` for building web version

**Benefits**:
- **Flexible**: Deploy as desktop app or web app (or both)
- **Single codebase**: Maintain one UI for multiple platforms
- **Fast development**: Native builds are faster during development
- **Best of both worlds**: Native performance + web accessibility

## Development Phases

**Phase 1** (✅ COMPLETED): Basic Slint UI structure
- ✅ Application skeleton with sidebar navigation (8 menu items)
- ✅ Multiple page structures implemented (About, Titles, Authors, Publishers, Genres, Locations)
- ✅ Slint component architecture established
- ✅ ScrollView integration for responsive content

**Phase 2** (🔄 80% COMPLETE): Core functionality
- ✅ MariaDB database integration with SQLx (13 migrations applied)
- ✅ Title management (create, read, update - delete missing)
- ✅ Basic CRUD operations with repository pattern
- ✅ Connection pooling and health checks
- 🔄 Volume management (database schema ready, handlers and UI needed)
- 🔄 Title-Author relationship (junction table ready, handlers needed)

**Phase 3** (🔄 40% COMPLETE): Advanced features
- ✅ Author management (full CRUD operations working)
- ✅ Publisher management (full CRUD operations working)
- ✅ Genre management (full CRUD operations working)
- ✅ Locations management (full CRUD with hierarchical structure)
- ⏳ Series management (not started)
- 🔄 Multiple copies per title (database ready, implementation needed)
- ⏳ Barcode scanning integration (not started)
- ⏳ Loan management system (database schema ready, implementation needed)
- ⏳ Borrower management (database schema ready, implementation needed)

**Phase 4** (⏳ NOT STARTED): Polish and extras
- ⏳ Dewey classification UI
- ⏳ Duplicate detection algorithms
- ⏳ Import/export functionality (CSV, JSON)
- ⏳ French/English internationalization (infrastructure ready with @tr())
- ⏳ Statistics and reporting dashboard
- ⏳ Search and filter capabilities
- ⏳ Cover image upload and display
- ⏳ Barcode generation (Code 128)

**Current Status**: Mid-Phase 2/Early Phase 3. Core infrastructure is solid, basic entities are fully functional, but the critical Title/Volume relationship and loan management system are not yet implemented.
