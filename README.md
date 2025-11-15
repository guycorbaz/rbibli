# rbibli - Personal Library Management System

A modern, full-featured personal library management system built entirely in Rust using Slint for the user interface. Designed for small-scale use (friends and family) with a focus on simplicity, data integrity, and flexible deployment options.

## ✨ Features

### Currently Implemented (Phase 3 - ~75% Complete)

- **📚 Title Management** - Full CRUD operations with ISBN lookup via Google Books API
- **📖 Volume Management** - Track multiple physical copies per title with unique barcodes
- **✍️ Author Management** - Complete biographical information and title associations
- **🏢 Publisher Management** - Company details and catalogs
- **🎭 Genre Management** - Categorize your collection
- **📍 Location Management** - Hierarchical storage organization (House > Room > Shelf)
- **👥 Borrower Management** - Track who you lend books to
- **👪 Borrower Groups** - Organize borrowers with custom loan policies
- **📅 Loan Management** - Barcode-based lending with automatic due dates
- **📊 Statistics Dashboard** - Visual analytics for your library:
  - Library overview (titles, volumes, authors, genres, active/overdue loans)
  - Volumes per genre with bar charts
  - Volumes per location
  - Loan status breakdown
- **🔍 Dewey Decimal Classification** - Search and browse DDC system
- **🌐 Multi-language Support** - Internationalization infrastructure ready

### Architecture Highlights

- **Title/Volume Separation** - Titles represent abstract book metadata, volumes are physical copies
- **Barcode System** - Code 128 format for volume tracking (VOL-XXXXXX)
- **Hierarchical Locations** - Organize storage with parent-child relationships
- **Trust-Based Lending** - Simple loan management without complex restrictions
- **Cross-Platform UI** - Native desktop now, WASM for web deployment later

## 🛠️ Technology Stack

### Frontend
- **[Slint 1.14.1](https://slint.dev/)** - Declarative UI framework for native and web
- **Rust** - Type-safe, memory-safe systems programming language
- **Reqwest** - HTTP client for API communication

### Backend
- **[Actix-web 4.11.0](https://actix.rs/)** - High-performance web framework
- **[Tokio 1.47.1](https://tokio.rs/)** - Async runtime
- **[SQLx](https://github.com/launchbadge/sqlx)** - Compile-time verified SQL queries
- **MariaDB** - Production-grade relational database

### Development Approach
- **Native-first** - Develop as desktop app for faster iteration
- **WASM-ready** - Same codebase will compile to WebAssembly for browser deployment
- **Client-Server** - REST API backend with native/web frontend options

## 🚀 Getting Started

### Prerequisites

- **Rust** (latest stable) - [Install from rustup.rs](https://rustup.rs/)
- **MariaDB/MySQL** - [Installation guide](documentation/sqlx_installation.md)
- **SQLx CLI** - For database migrations:
  ```bash
  cargo install sqlx-cli --no-default-features --features mysql
  ```

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/rbibli.git
   cd rbibli
   ```

2. **Set up the database**

   Create a `.env` file in the `backend/` directory:
   ```env
   DATABASE_URL=mysql://username:password@localhost:3306/rbibli
   HOST=127.0.0.1
   PORT=8000
   ```

3. **Run database migrations**
   ```bash
   cd backend
   sqlx database create
   sqlx migrate run
   ```

4. **Start the backend**
   ```bash
   cargo run --release
   ```
   The API will be available at `http://127.0.0.1:8000`

5. **Start the frontend** (in a new terminal)
   ```bash
   cd frontend
   cargo run --release
   ```

## 📖 Usage

### Main Features

1. **Manage Your Collection**
   - Add titles with ISBN lookup for automatic metadata
   - Create multiple volumes (physical copies) per title
   - Organize volumes in hierarchical storage locations

2. **Track Loans**
   - Scan barcodes to check out books
   - Automatic due date calculation based on borrower groups
   - Visual indicators for overdue items

3. **View Statistics**
   - See which genres dominate your collection
   - Track where volumes are located
   - Monitor active and overdue loans

### Navigation

- **Titles** - Browse and manage your book collection
- **Locations** - Organize physical storage spaces
- **Authors** - Manage author information
- **Publishers** - Track publishing companies
- **Genres** - Categorize your books
- **Loans** - Check out, return, and manage loans
- **Statistics** - View library analytics
- **About** - Application information

## 📁 Project Structure

```
rbibli/
├── frontend/           # Slint UI application
│   ├── src/           # Rust source code
│   │   ├── main.rs    # Entry point
│   │   ├── models.rs  # Data structures
│   │   └── api_client.rs  # HTTP client
│   └── ui/            # Slint UI files
│       ├── app-window.slint
│       ├── pages/     # Page components
│       └── side_bar.slint
├── backend/           # REST API server
│   ├── src/
│   │   ├── main.rs    # Server entry point
│   │   ├── handlers/  # API endpoints
│   │   └── models/    # Database models
│   └── migrations/    # SQLx database migrations
└── documentation/     # Detailed documentation
    ├── requirements.md
    ├── architecture.md
    ├── api.md
    └── ...
```

## 📚 Documentation

Comprehensive documentation is available in the [`documentation/`](documentation/) folder:

- **[Requirements](documentation/requirements.md)** - Complete functional specifications
- **[Architecture](documentation/architecture.md)** - Technical design and data models
- **[API Documentation](documentation/api.md)** - REST API endpoints
- **[Development Setup](documentation/development_environment.md)** - Detailed setup instructions
- **[Planning](documentation/planning.md)** - Development roadmap
- **[CLAUDE.md](documentation/CLAUDE.md)** - Claude Code integration guide

## 🎯 Development Status

**Current Phase: Late Phase 3 (75% Complete)**

### ✅ Completed
- Database integration with 13 migrations
- Full CRUD for titles, authors, publishers, genres, locations
- Volume management with barcode support
- Complete loan management system
- Statistics dashboard with visualizations
- ISBN lookup integration
- Dewey Decimal Classification search

### 🔄 In Progress
- Title-Author relationship management
- Series management

### ⏳ Planned (Phase 4)
- Barcode generation (Code 128)
- Import/export functionality (CSV, JSON)
- Duplicate detection algorithms
- Cover image upload and display
- WASM build target for web deployment
- Progressive Web App (PWA) features

## 🔑 Key Concepts

### Title vs Volume

The system makes a crucial distinction:
- **Title** - Abstract book metadata (ISBN, authors, genre, summary)
- **Volume** - Physical copy with unique barcode, condition, location

This allows:
- Wishlist functionality (titles with 0 volumes)
- Multiple copies tracking
- Individual volume notes and locations

### Barcode System

- **Volume Barcodes** - `VOL-000001` (Code 128) for physical copies
- **ISBN Barcodes** - Standard EAN-13 for title identification

### Loan Management

- Title-based requests, volume-specific fulfillment
- Automatic due date calculation based on title type
- Simple, trust-based system (no fines or suspensions)

## 🤝 Contributing

This is currently a personal project. Feel free to fork and adapt for your own use!

## 📄 License

Licensed under the MIT License - see [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- **[Slint](https://slint.dev/)** - Cross-platform UI framework
- **[Actix-web](https://actix.rs/)** - Web framework
- **[SQLx](https://github.com/launchbadge/sqlx)** - SQL toolkit
- **Google Books API** - ISBN metadata lookup

## 📬 Contact

For questions or suggestions, please open an issue on GitHub.

---

**Built with ❤️ using Rust and Slint**
