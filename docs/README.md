# rbibli

### Personal Library Management System

A modern, elegant solution for managing your personal book collection. Built entirely in Rust with a beautiful native interface.

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

## 🖼️ Screenshots

### Library Overview
![Statistics Dashboard](screenshots/statistics-dashboard.png)
*View your entire collection with beautiful visual analytics*

### Book Management
![Title Management](screenshots/title-management.png)
*Easily add and organize your books with rich metadata*

### Loan Tracking
![Loan Management](screenshots/loan-management.png)
*Track who borrowed what and when it's due*

---

## 🚀 Key Features

### 📚 Complete Library Management
- Full book metadata (title, subtitle, ISBN, pages, language, summary)
- Author and publisher information
- Genre categorization
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
- Search and filter capabilities (coming soon)

---

## 🎨 Technology

Built with modern, reliable technology:

- **[Rust](https://www.rust-lang.org/)** - Memory-safe, blazingly fast
- **[Slint](https://slint.dev/)** - Beautiful native UI framework
- **[MariaDB](https://mariadb.org/)** - Reliable data storage
- **REST API** - Clean architecture with actix-web

---

## 💡 Use Cases

### Home Library
Perfect for managing your personal book collection at home. Track exactly where each book is located and who borrowed it.

### Small Library/Bookshop
Organize inventory, manage loans, and keep track of your collection with professional-grade tools.

### Book Collectors
Maintain detailed records of your collection, including condition, acquisition details, and multiple editions.

### Lending Libraries
Share your collection with friends and family while keeping track of who has what and when it's due back.

---

## 🌟 What Makes rbibli Special?

### Title vs Volume Architecture

Unlike simple cataloging apps, rbibli understands that **one book title can have multiple physical copies**:

- **Titles** = The abstract book (metadata, authors, genre)
- **Volumes** = Individual physical copies with barcodes

This means you can:
- ✅ Track multiple copies of your favorite books
- ✅ Maintain a wishlist (titles with no volumes yet)
- ✅ Loan out specific copies while keeping others available
- ✅ Track condition per physical copy

### Trust-Based System

Designed for personal use with friends and family:
- No complex permissions or restrictions
- Simple, straightforward loan management
- Focus on tracking, not enforcement

---

## 📥 Get Started

### Installation

1. **Download** the latest release for your platform (Windows, macOS, Linux)
2. **Install MariaDB** (or MySQL)
3. **Run the application** - It will guide you through initial setup
4. **Start cataloging** your books!

[View Installation Guide →](https://github.com/guycorbaz/rbibli#installation)

### Quick Start

Once installed:
1. Add your first books by ISBN or manually
2. Create storage locations for your shelves
3. Add physical volumes with barcodes
4. Optionally add borrowers for loan tracking

[View Full Documentation →](https://github.com/guycorbaz/rbibli/tree/main/documentation)

---

## 🗺️ Roadmap

**Current Status: 85% Complete** - Fully functional for daily use!

### ✅ Available Now
- Complete library cataloging
- Volume tracking with barcodes
- Loan management system
- Statistics dashboard
- ISBN lookup
- Dewey classification

### 🔄 Coming Soon
- Series management
- Advanced search and filters
- Import/export (CSV, JSON)
- Web-based access (WASM)

### 💭 Future Ideas
- Mobile app
- Barcode scanner integration
- Community features
- Library sharing

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

## 🎉 Start Organizing Today

Ready to bring order to your book collection?

[**Download rbibli**](https://github.com/guycorbaz/rbibli/releases) | [**View on GitHub**](https://github.com/guycorbaz/rbibli) | [**Read Documentation**](https://github.com/guycorbaz/rbibli/tree/main/documentation)

---

<div align="center">

**Built for book lovers**

Made with [Rust](https://www.rust-lang.org/) and [Slint](https://slint.dev/)

[⭐ Star on GitHub](https://github.com/guycorbaz/rbibli) • [🐦 Share on Twitter](https://twitter.com/intent/tweet?text=Check%20out%20rbibli%20-%20A%20beautiful%20personal%20library%20management%20system!)

</div>
