---
layout: default
title: rbibli - Personal Library Manager
description: A modern, open-source personal library management system built with Rust and Slint. Catalog books, track loans, and manage your collection.
keywords: rust, library manager, personal library, book catalog, slint, open source, mariadb
permalink: /
---


# rbibli

## Personal Library Management System

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

You can run rbibli using Docker. Official images are available on Docker Hub:
[https://hub.docker.com/r/gcorbaz/rbibli](https://hub.docker.com/r/gcorbaz/rbibli)

Example `docker-compose.yml`:

```yaml
services:
  rbibli:
    image: gcorbaz/rbibli:latest
    ports:
      - "8080:8080"
    environment:
      # Database Connection
      # Note: Use APP__ prefix (double underscore) for configuration
      - APP__DATABASE__URL=mysql://user:password@db:3306/rbibli
```

**Configuration**: The application is configured entirely via environment variables.

### Installation

1. **Clone the repository**

   ```bash
   git clone https://github.com/guycorbaz/rbibli.git
   cd rbibli
   ```

2. **Set up the database**
   **a. Configuration**
   Create a `.env` file in the project root:

   ```env
   DATABASE_URL=mysql://rbibli:your_password@127.0.0.1:3306/rbibli
   HOST=127.0.0.1
   PORT=8000
   ```

3. **Run migrations**

   ```bash
   cd backend
   sqlx database create
   sqlx migrate run
   ```

4. **Run the application**

   **Option A: Docker**

   ```bash
   docker compose up --build
   ```

   **Option B: Manual**
   Start backend: `cargo run --release` (in `backend/`)
   Start frontend: `trunk serve --release` (in `frontend/`)

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

### Built for book lovers

Made with [Rust](https://www.rust-lang.org/) and [Slint](https://slint.dev/)

[⭐ Star on GitHub](https://github.com/guycorbaz/rbibli)
