# Functional Specifications - Personal Library Manager

## Overview
Web application for managing a personal library with a modern interface, barcode system, and complete loan management.

## 1. Volume Management

### 1.1 Basic Information
- **Main metadata**: title, subtitle, ISBN, publisher, year of publication
- **Classification**: genre, sub-genre, language, number of pages
- **Dewey Classification**: 3-digit code (e.g., 796.332 for football)
- **Dewey Category**: automatic label (e.g., "Sports and Leisure")
- **Description**: summary, keywords, personal notes
- **Cover**: image upload or automatic retrieval via ISBN
- **Physical location**: shelf, room, storage box

### 1.2 Volume Conditions
- **Excellent**: new or like-new
- **Good**: slight signs of wear
- **Fair**: visible wear but readable
- **Poor**: very worn, withdrawn from loans
- **Lost**: misplaced or not returned

### 1.3 Unique Numbering System
- **Unique identifier**: automatic generation (format: BIB-XXXXXX)
- **Barcode**: Code 128 or EAN-13 format
- **QR Code**: alternative for smartphones
- **Printable labels**: standard formats (Avery, Brother)

## 2. Author Management

### 2.1 Author Information
- **Identity**: last name, first name, pseudonyms
- **Biography**: dates, nationality, short biography
- **Media**: photo, external links (website, social networks)
- **Roles**: main author, co-author, illustrator, translator, preface writer

### 2.2 Relationships
- **Complete bibliography**: all works by the author
- **Series by author**: automatic grouping
- **Collaborations**: frequent co-authors

## 3. Series Management

### 3.1 Organization
- **Series name**: main title and sub-series
- **Numbering**: volume, issue, reading order
- **Status**: ongoing, completed, abandoned
- **Completeness**: owned volumes vs existing volumes

### 3.2 Features
- **Wishlist**: missing volumes to acquire
- **New release alerts**: notification of new publications
- **Duplicate prevention**: check before purchase

## 4. Loan Management

### 4.1 Borrowers
- **Address book**: name, phone, email, address
- **Profiles**: reliability, reading preferences, history
- **Custom limits**: max number of loans, duration by person
- **Status**: active, suspended, blacklist

### 4.2 Loan System
- **Simple loan**: volume, borrower, dates
- **Configurable duration**: by document type or borrower
- **Notifications**: automatic reminders before due date
- **Complete history**: traceability of all loans

### 4.3 Loan Status
- **Available**: in library
- **Loaned**: with a borrower
- **Overdue**: past return date
- **Lost**: declared lost by borrower

## 5. Scanner Interface

### 5.1 Scan Modes
- **Loan mode**: scan volume + scan borrower card
- **Return mode**: scan volume for automatic return
- **Inventory mode**: presence check
- **Location mode**: quickly find a volume

### 5.2 Dedicated Interface
- **Simplified screen**: optimized for scanner use
- **Visual/audio feedback**: confirmation of actions
- **Error handling**: unrecognized codes, already loaned volumes

## 6. Search and Navigation

### 6.1 Advanced Search
- **Multiple criteria**: title, author, series, genre, ISBN
- **Filters**: availability, condition, location, date added
- **Full-text search**: in summaries and notes
- **Saved searches**: frequent queries

### 6.2 Navigation
- **Alphabetical index**: by title, author, series
- **Genre navigation**: hierarchical classification
- **Dewey navigation**: browse tree by domains
- **Latest additions**: library’s newest entries
- **Suggestions**: based on reading history

## 7. Lists and Collections

### 7.1 Predefined Lists
- **All volumes**: complete catalog
- **Loaned volumes**: with borrower and dates
- **Overdue volumes**: past due date
- **Volumes by series**: organized by collection
- **New arrivals**: latest additions

### 7.2 Custom Lists
- **Favorites**: preferred volumes
- **To re-read**: personal reading list
- **Recommendations**: to suggest to friends
- **Wishlist**: volumes to acquire

## 8. Statistics and Reports

### 8.1 Dashboards
- **Overview**: total number of volumes, ongoing loans
- **Charts**: collection growth, loans per month
- **Top lists**: most loaned volumes, favorite authors

### 8.2 Detailed Reports
- **Inventory**: full collection status
- **Loan history**: by volume, by borrower
- **Never loaned volumes**: identify “forgotten” items
- **Budget analysis**: collection cost, loan ROI

## 9. Technical Features

### 9.1 Web Interface
- **Responsive design**: mobile/tablet/desktop adaptation
- **Themes**: customizable appearance
- **Keyboard shortcuts**: quick navigation
- **Offline mode**: consult without internet

### 9.2 Import/Export
- **Supported formats**: CSV, JSON, XML
- **Automatic backup**: regular data backup
- **Cloud sync**: multi-device access
- **Migration**: import from other software

### 9.3 External Integrations
- **Bibliographic APIs**: Google Books, OpenLibrary
- **Automatic retrieval**: metadata via ISBN
- **Price comparison**: purchase assistance
- **Notifications**: email, SMS reminders

## 10. Multilingual Support

### 10.1 Supported Languages
- **Main languages**: French, English, Spanish, German, Italian
- **User interface**: full translation of menus and messages
- **Metadata**: special character support (accents, Cyrillic, etc.)
- **Date formats**: adaptation by locale (DD/MM/YYYY vs MM/DD/YYYY)
- **Numeric formats**: decimal/thousand separators by region

### 10.2 Features
- **Language selector**: on-the-fly interface change
- **Automatic detection**: browser default language
- **Multilingual Dewey classification**: translated labels
- **Multilingual search**: search across all languages
- **Localized notifications**: emails/messages in chosen language

### 10.3 Content Management
- **Multilingual volumes**: original title + translations
- **International authors**: names in various alphabets
- **Localized genres**: culturally adapted classifications
- **Documentation**: translated help and tutorials
- **Error messages**: fully translated system feedback

## 11. Security and Maintenance

### 11.1 Authentication
- **Secure access**: login/password
- **Access levels**: admin, user, guest
- **Session management**: automatic timeout

### 11.2 Backup and Recovery
- **Automatic backup**: daily, weekly
- **Restoration**: recovery in case of issues
- **Audit trail**: change traceability
- **Versioning**: change history

## 12. Dewey Classification

### 12.1 Classification System
- **Standard codes**: 000–999 according to Dewey Decimal Classification
- **Hierarchy**: class (hundreds), division (tens), section (units)
- **French labels**: translation of main categories
- **Mixed classification**: Dewey + custom genres
- **Sub-classification**: additional precision if needed

### 12.2 Features
- **Automatic assignment**: suggestion based on genre/subject
- **Manual assignment**: direct code entry
- **Validation**: check code validity
- **Classification search**: filter by Dewey codes
- **Optimized shelving**: logical order for physical organization

### 12.3 Advantages
- **Universal organization**: recognized international standard
- **Physical shelving**: logical shelf order
- **Easier search**: by domain of knowledge
- **Compatibility**: with other library systems
- **Scalability**: extendable, precise system

## 13. Future Enhancements

### 13.1 Social Features
- **List sharing**: with other users
- **Cross-recommendations**: between libraries
- **Book clubs**: group management

### 13.2 Artificial Intelligence
- **Personalized suggestions**: based on preferences
- **Image recognition**: cover scanning
- **Predictive analysis**: purchase trends

## Development Priorities

### Phase 1 (MVP)
- Basic volume management
- Simple loan system
- Responsive web interface
- Barcodes and scanner

### Phase 2
- Author and series management
- Dewey classification
- Basic multilingual support
- Advanced search
- Basic statistics
- Import/export

### Phase 3
- Full multilingual support
- Advanced features
- External integrations
- Performance optimizations
- Social features
