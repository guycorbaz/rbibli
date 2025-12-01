# rbibli

### Personal Library Management System

A modern, elegant solution for managing your personal book collection. Built entirely in Rust with a beautiful native interface using Slint.

**Perfect for book lovers who want to organize their home library with ease.**

---

## ✨ What is rbibli?

**rbibli** helps you catalog, organize, and track your personal book collection. Whether you have dozens or thousands of books, rbibli makes it easy to:

- 📚 **Catalog your books** with rich metadata (ISBN, authors, genres, cover images)
- 📍 **Track physical locations** with hierarchical storage (Room > Bookshelf > Shelf)
- 👥 **Manage loans** to friends and family with automatic due dates
- 📊 **Visualize your collection** with insightful statistics and charts
- 🔍 **Search and organize** by genre, author, publisher, or location

---

## 🎯 Why rbibli?

### Simple Yet Powerful

- **Easy to use** - Clean, intuitive interface that makes cataloging enjoyable
- **Fast and responsive** - Native desktop application with instant updates
- **Flexible** - Track multiple copies of the same book, organize by location, manage loans

### Smart Features

- **ISBN Lookup** - Automatic book metadata from Google Books
- **Barcode Support** - Quick checkout with volume barcodes
- **Dewey Classification** - Professional library organization
- **Statistics Dashboard** - See your collection at a glance

### Built for You

- **Privacy-focused** - Your data stays on your computer
- **No subscriptions** - Free and open-source
- **Customizable** - Organize your way with custom locations and borrower groups

---

## 🚀 Key Features

### 📚 Complete Library Management
- Full book metadata (title, subtitle, ISBN, pages, language, summary)
- Author and publisher information
- Genre and Series categorization
- Dewey Decimal Classification
- Cover image storage

### 📖 Multiple Physical Copies
- Track multiple volumes of the same title
- Individual condition tracking (excellent → damaged)
- Unique barcode for each physical copy
- Location assignment per volume

### 📍 Hierarchical Storage
- Organize by room, bookshelf, and shelf
- Full path display ("Office > Bookshelf A > Shelf 3")
- Easy navigation of your physical space

### 👥 Smart Loan Management
- Simple borrower management
- Borrower groups with custom loan policies
- Automatic due date calculation
- Visual overdue highlighting
- Barcode-based checkout/return

### 📊 Insightful Analytics
- Library overview (total books, active loans, overdue items)
- Volumes per genre with bar charts
- Volumes per location
- Loan activity tracking

### 🔍 Advanced Organization
- ISBN metadata lookup via Google Books API
- Complete Dewey Decimal Classification system

---

## 🎨 Technology

Built with modern, reliable technology:

- **[Rust](https://www.rust-lang.org/)** - Memory-safe, blazingly fast
- **[Slint](https://slint.dev/)** - Beautiful native UI framework
- **[MariaDB](https://mariadb.org/)** - Reliable data storage
- **[Actix-web](https://actix.rs/)** - High-performance web framework

---

## 📥 Get Started

### Prerequisites

- **Rust** (latest stable)
- **MariaDB/MySQL**
- **SQLx CLI** (`cargo install sqlx-cli`)

### Docker

You can also run rbibli using Docker. Official images are available on Docker Hub:
[https://hub.docker.com/r/gcorbaz/rbibli](https://hub.docker.com/r/gcorbaz/rbibli)

Example `docker-compose.yml`:
```yaml
services:
  backend:
    image: gcorbaz/rbibli:backend
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=mysql://user:password@db:3306/rbibli
    volumes:
      - ./config:/config
    command: ["backend", "--config", "/config/configuration.toml"]
  
  frontend:
    image: gcorbaz/rbibli:frontend
    ports:
      - "80:80"
```

**Configuration**: Create a `config` directory in the same folder as `docker-compose.yml` and place your `configuration.toml` file inside it. The backend will read the configuration from `/config/configuration.toml`.

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/guycorbaz/rbibli.git
   cd rbibli
   ```

2. **Set up the database**
   **a. Runtime Configuration**
   Create a `configuration.toml` file in `backend/`:
   ```toml
   [application]
   port = 8000
   host = "127.0.0.1"

   [database]
   username = "rbibli"
   password = "your_password"
   port = 3306
   host = "127.0.0.1"
   database_name = "rbibli"
   ```

   **b. Compile-time Configuration**
   Create a `.env` file in `backend/` (for SQLx):
   ```env
   DATABASE_URL=mysql://rbibli:your_password@127.0.0.1:3306/rbibli
   ```

3. **Run migrations**
   ```bash
   cd backend
   sqlx database create
   sqlx migrate run
   ```

4. **Run the application**
   Start backend: `cargo run --release` (in `backend/`)
   Start frontend: `cargo run --release` (in `frontend/`)

[View Full Documentation →](https://github.com/guycorbaz/rbibli/tree/main/documentation)

---

## 🗺️ Roadmap

**Current Status: Phase 3 (88% Complete)**

### ✅ Available Now
- Complete library cataloging
- Volume tracking with barcodes
- Loan management system
- Statistics dashboard
- ISBN lookup
- Dewey classification (Manual)
- Series management

### 🔄 In Progress / Coming Soon
- Web-based access (WASM)
- Advanced search and filters
- Import/export (CSV, JSON)
- Barcode scanner integration

[View Detailed Roadmap →](https://github.com/guycorbaz/rbibli/blob/main/documentation/planning.md)

---

## 🤝 Community

### Support

- 📖 [Documentation](https://github.com/guycorbaz/rbibli/tree/main/documentation)
- 💬 [Discussions](https://github.com/guycorbaz/rbibli/discussions)
- 🐛 [Report Issues](https://github.com/guycorbaz/rbibli/issues)

### Contributing

rbibli is open-source and welcomes contributions! Whether you're fixing bugs, adding features, or improving documentation - we'd love your help.

[Contribution Guide →](https://github.com/guycorbaz/rbibli#contributing)

---

## 📄 License

rbibli is free and open-source software licensed under the MIT License.

[View License →](https://github.com/guycorbaz/rbibli/blob/main/LICENSE)

---

<div style="text-align: center;">

**Built for book lovers**

Made with [Rust](https://www.rust-lang.org/) and [Slint](https://slint.dev/)

[⭐ Star on GitHub](https://github.com/guycorbaz/rbibli)

</div>
