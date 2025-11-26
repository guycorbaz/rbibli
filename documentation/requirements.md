# Functional Specifications - Personal Library Manager

## Overview
Personal library management application with a modern interface, barcode system, and complete loan management.

> **Note**: This document describes functional requirements independent of implementation technology. The application is implemented with Slint UI (native, with WASM compilation planned) and a REST API backend (actix-web + MariaDB). See `CLAUDE.md` and `planning.md` for current architecture.

## 1. Title Management

### 1.1 Title Information
- **Main metadata**: title, subtitle, ISBN, publisher, year of publication
- **Classification**: genre, sub-genre, language, number of pages
- **Dewey Classification**: 3-digit code (e.g., 796.332 for football)
- **Dewey Category**: automatic label (e.g., "Sports and Leisure")
- **Description**: summary, keywords, personal notes
- **Cover**: image upload or automatic retrieval via ISBN
- **Authors**: one or multiple authors associated with the title

### 1.2 Title-Volume Relationship
- **One title can have 0 to n physical volumes**
- **Each volume belongs to exactly one title**
- **Volumes share the title's metadata but have individual properties**
- **Independent volume management**: condition, location, barcode
- **Wishlist support**: titles with 0 volumes for future acquisitions

## 2. Volume Management (Physical Copies)

### 2.1 Volume Properties
- **Title reference**: link to parent title
- **Copy number**: sequential number (1, 2, 3...)
- **Physical condition**: excellent, good, fair, poor, damaged
- **Physical location**: shelf, room, storage box
- **Loan status**: available, loaned, overdue, lost, reserved
- **Individual notes**: specific to this copy

### 2.2 Physical Conditions
- **Excellent**: new or like-new
- **Good**: slight signs of wear
- **Fair**: visible wear but readable
- **Poor**: very worn, should not be loaned
- **Damaged**: requires repair before use

### 2.3 Loan Status
- **Available**: in library and ready to loan
- **Loaned**: currently with a borrower
- **Overdue**: past return date
- **Lost**: declared lost by borrower or library
- **Maintenance**: temporarily unavailable for repair or processing

### 2.4 Barcode Management System
- **Volume identifier**: automatic generation (format: VOL-000001, sequential)
- **Internal barcode format**: Code 128 for volume-specific identification
- **ISBN barcode support**: EAN-13 format for existing book ISBN codes
- **Each volume has its own unique internal barcode**
- **No shared barcodes between volumes of the same title**
- **Dual barcode strategy**: 
  - ISBN (EAN-13): for title identification and metadata retrieval
  - Volume code (Code 128): for specific volume tracking and operations
- **Printable labels**: standard formats (Avery 5160, Brother DK-1201)
- **Validation**: check digit verification for manual entry
- **Duplicate prevention**: system-wide uniqueness enforcement for internal codes

## 3. Author Management

### 3.1 Author Information
- **Identity**: last name, first name, pseudonyms
- **Biography**: dates, nationality, short biography
- **Media**: photo, external links (website, social networks)
- **Roles**: main author, co-author, illustrator, translator, preface writer

### 3.2 Relationships
- **Complete bibliography**: all titles by the author
- **Series by author**: automatic grouping
- **Collaborations**: frequent co-authors
- **Title associations**: link authors to titles (not individual volumes)

## 4. Series Management

### 4.1 Organization
- **Series name**: main title and sub-series
- **Numbering**: volume, issue, reading order
- **Status**: ongoing, completed, abandoned
- **Completeness**: owned titles vs existing titles in series

### 4.2 Features
- **Wishlist**: missing titles to acquire
- **New release alerts**: notification of new publications
- **Duplicate prevention**: check before purchase
- **Volume tracking**: track which titles have physical copies

## 5. Loan Management

### 5.1 Borrowers (Simple)
- **Contact information**: name, phone, email
- **Basic tracking**: simple loan history
- **No complex profiles**: no limits, restrictions, or status management
- **Trust-based system**: designed for friends, family, small groups

### 5.2 Loan System
- **Title-based loans**: loan a title, system selects best available volume
- **Volume tracking**: record which specific volume (barcode) is loaned
- **Notifications**: automatic reminders before due date
- **Complete history**: traceability of all loans by title and volume

### 5.2.1 Title Types for Loan Duration
- **Fiction**: novels, short stories (default 21 days)
- **Non-fiction**: essays, biographies (default 14 days) 
- **Reference**: dictionaries, encyclopedias (default 7 days)
- **Magazines**: periodicals (default 3 days)
- **Rare books**: special collection (default 7 days, no renewal)
- **Custom types**: user-defined categories with configurable durations

### 5.3 Loan Management Rules (Simplified)
- **Loan duration**: configurable default (14 days), simple extension possible
- **No complex rules**: no automatic restrictions, penalties, or suspensions
- **Manual management**: owner decides on extensions, overdue handling
- **Lost item tracking**: simple marking as "lost", no financial calculations
- **Trust-based approach**: suitable for personal use and trusted borrowers

### 5.4 Volume Selection Logic
- **Availability check**: verify at least one volume is available for the title
- **Automatic selection**: choose first available volume (by copy number)
- **Manual selection**: allow librarian to choose specific volume if needed
- **Condition filtering**: prefer volumes in better condition, exclude "maintenance" status
- **Location preference**: prioritize volumes in accessible locations
- **Reservation priority**: when fulfilling reservations, apply same selection logic
- **Copy tracking**: record which specific volume (barcode) is loaned

### 5.5 Title/Volume Workflow
- **Loan creation**: borrower selects title, system assigns best available volume
- **Return process**: scan volume barcode, system identifies title and loan
- **Reservation system**: reserve titles (not specific volumes)
- **Availability display**: show "2/3 copies available" for titles with multiple volumes

## 6. Scanner Interface

### 6.1 Scan Modes
- **Loan mode**: scan volume barcode (Code 128) + scan borrower card
- **Return mode**: scan volume barcode (Code 128) for automatic return
- **Add title mode**: scan ISBN barcode (EAN-13) to add new title with metadata
- **Search mode**: scan ISBN barcode (EAN-13) to find existing title in collection
- **Inventory mode**: scan volume barcode (Code 128) for presence check
- **Location mode**: scan volume barcode (Code 128) to quickly find a volume

### 6.2 Dedicated Interface
- **Simplified screen**: optimized for scanner use with large buttons
- **Visual/audio feedback**: confirmation beeps, color-coded responses
- **Error handling**: unrecognized codes, already loaned volumes, damaged barcodes
- **Dual barcode support**: automatic detection of Code 128 (volumes) and EAN-13 (ISBN)
- **Context-aware actions**: 
  - ISBN scan: search/add title, retrieve metadata
  - Volume scan: loan/return/inventory operations
- **Offline mode**: queue actions when network unavailable
- **Touch interface**: tablet-optimized for manual entry backup
- **Accessibility**: screen reader compatible, high contrast mode

### 6.3 Title/Volume Context Display
- **Volume scan result**: show volume details with parent title information
- **Title context**: display "Volume 2/3 of [Title Name]"
- **Availability status**: show other copies status when scanning one volume
- **Smart loan interface**: when scanning volume, show title loan options
- **Return confirmation**: display both volume and title information

## 7. Search and Navigation

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

## 8. Lists and Collections

### 8.1 Title Lists
- **All titles**: complete catalog with volume counts and availability
- **Available titles**: titles with at least one available volume
- **Wishlist**: titles with 0 volumes for future acquisition
- **Incomplete series**: series missing some titles
- **New title arrivals**: recently added titles
- **Popular titles**: most frequently loaned titles

### 8.2 Volume Lists
- **All volumes**: individual physical copies with status
- **Available volumes**: ready to loan
- **Loaned volumes**: currently with borrowers
- **Overdue volumes**: past due date
- **Damaged volumes**: requiring repair or replacement
- **New volume arrivals**: recently added physical copies

### 8.3 Custom Lists
- **Favorite titles**: preferred titles for quick access
- **Reading queue**: personal reading list
- **Recommendations**: titles to suggest to friends
- **Acquisition priorities**: titles to purchase next
- **Duplicate needs**: titles requiring additional volumes
- **Location-based**: titles by physical location

## 9. Statistics and Reports

### 8.1 Dashboards
- **Overview**: total number of volumes, ongoing loans
- **Charts**: collection growth, loans per month
- **Top lists**: most loaned volumes, favorite authors

### 8.2 Detailed Reports
- **Inventory**: full collection status
- **Loan history**: by volume, by borrower
- **Never loaned volumes**: identify “forgotten” items
- **Budget analysis**: collection cost, loan ROI

## 10. Technical Features

### 10.1 User Interface
- **Web application**: Browser-based, accessible from any device
- **Responsive design**: Adapts to desktop, tablet, and mobile screens
- **Modern browsers**: Chrome, Firefox, Safari, Edge support
- **Themes**: customizable appearance
- **Keyboard shortcuts**: quick navigation
- **Progressive Web App** (planned): Offline capability and install-to-device

### 10.1.1 Copy Management Interface
- **Title detail page**: expandable section showing all volumes
- **Add copy button**: prominent button to add new volume to title
- **Volume grid view**: visual grid showing all copies with status indicators
- **Individual copy actions**: edit, delete, move, change condition per volume
- **Bulk copy operations**: select multiple volumes for batch actions
- **Copy status indicators**: color-coded availability, condition, location
- **Quick loan interface**: loan specific copy or let system choose best available

### 10.1.2 Reservation Management Interface
- **Reservation queue**: visual queue showing waiting list for titles
- **User reservation panel**: personal reservations with status and estimated availability
- **Admin reservation management**: view, modify, prioritize, cancel reservations
- **Notification center**: alerts for available reserved titles
- **Reservation history**: track past reservations and fulfillment times

### 10.2 Import/Export

#### 10.2.1 Import Formats and Structure
- **CSV format**: hierarchical structure with title and volume rows
  ```csv
  Type,Title,ISBN,Author,Publisher,Year,VolumeBarcode,Condition,Location
  TITLE,Harry Potter 1,9780439708180,J.K. Rowling,Scholastic,1997,,,
  VOLUME,,,,,VOL-000001,excellent,A1-S2
  VOLUME,,,,,VOL-000002,good,A1-S2
  TITLE,1984,9780451524935,George Orwell,Signet,1949,,,
  VOLUME,,,,,VOL-000003,fair,B2-S1
  ```
- **JSON format**: nested structure preserving title/volume hierarchy
  ```json
  {
    "titles": [
      {
        "title": "Harry Potter 1",
        "isbn": "9780439708180",
        "author": "J.K. Rowling",
        "volumes": [
          {"barcode": "VOL-000001", "condition": "excellent", "location": "A1-S2"},
          {"barcode": "VOL-000002", "condition": "good", "location": "A1-S2"}
        ]
      }
    ]
  }
  ```

#### 10.2.2 Import Operations
- **Title import**: create titles with automatic first volume generation
- **Volume import**: add volumes to existing titles by ISBN matching
- **Bulk operations**: import multiple copies of the same title
- **Duplicate handling**: skip, merge, or create new based on user preference
- **Validation**: verify ISBN, barcode uniqueness, required fields
- **Error reporting**: detailed log of import issues with line numbers

#### 10.2.3 Export Options
- **Complete hierarchy**: titles with all associated volumes
- **Titles only**: metadata without volume details
- **Volumes only**: physical inventory with title references
- **Filtered exports**: by date range, location, condition, availability
- **Format options**: CSV, JSON, XML with customizable field selection

#### 10.2.4 Migration Tools
- **Field mapping**: map source fields to target schema
- **Data transformation**: convert formats, normalize data
- **Preview mode**: show import results before committing
- **Rollback capability**: undo imports if issues detected

### 10.3 Simple External Integration
- **Single source**: Google Books API for metadata retrieval
- **Basic error handling**: fallback to manual entry if API unavailable
- **No complex authentication**: simple API usage without key management
- **Optional email**: basic email notifications if configured
- **Graceful degradation**: system works fully without external services

### 10.4 Backend API

The backend provides a REST API for all data operations. The API endpoints listed here represent the planned functionality. For complete and up-to-date API documentation, see `api.md`.

#### Essential Operations (Planned)
- `GET /api/v1/titles` - List titles with basic information
- `POST /api/v1/titles` - Create a title
- `GET /api/v1/titles/{id}` - Title details with volumes
- `PUT /api/v1/titles/{id}` - Update title
- `DELETE /api/v1/titles/{id}` - Delete title

#### Volume Management (Planned)
- `POST /api/v1/titles/{id}/volumes` - Add volume to title
- `PUT /api/v1/volumes/{id}` - Update volume
- `DELETE /api/v1/volumes/{id}` - Delete volume

#### Basic Loan Operations (Planned)
- `POST /api/v1/loans` - Create loan
- `PUT /api/v1/loans/{id}/return` - Return volume
- `GET /api/v1/loans/active` - Active loans

#### Scanner Support (Planned)
- `GET /api/v1/scan/volume/{barcode}` - Find volume by barcode
- `GET /api/v1/scan/isbn/{isbn}` - Find title by ISBN

## 11. Basic Multilingual Support

### 11.1 Essential Language Support
- **Primary languages**: French and English interface
- **Character support**: UTF-8 for accents and special characters
- **Simple localization**: basic date formats and number display

### 11.2 Simplified Features
- **Language toggle**: simple French/English interface switch
- **International content**: support for books in any language
- **Basic search**: search works across different character sets
- **No complex localization**: simplified approach for personal use

## 12. Security and Maintenance (Personal Use)

### 12.1 Basic Authentication
- **Simple access**: optional login/password for privacy protection
- **Session management**: basic session timeout (2 hours)
- **Local access**: designed for trusted local network use
- **Single user mode**: simplified interface without complex user management
- **Guest access**: optional read-only access for visitors

### 12.2 Data Protection
- **Input validation**: prevent data corruption and basic XSS attacks
- **File upload security**: basic type and size validation for images
- **HTTPS support**: optional SSL configuration for remote access
- **Data integrity**: basic validation to prevent accidental data loss
- **Safe operations**: confirmation dialogs for destructive actions

### 12.3 Backup and Recovery
- **Manual backup**: export/import functionality for data safety
- **Local backup**: simple file-based backup to local storage or external drive
- **Data export**: complete data export in JSON/CSV formats
- **Import validation**: verify data integrity during import operations
- **Recovery tools**: restore from backup files with conflict resolution
- **Portable data**: easy migration between installations

## 13. Dewey Classification (Simplified)

### 13.1 Classification System
- **Manual Entry**: Users can manually enter Dewey codes and category names
- **Standard codes**: Supports standard DDC format (e.g., 000–999.999)
- **Flexibility**: No strict validation against a database, allowing for custom or modified codes
- **Integration**: Fields available on Title entity

### 13.2 Title-Level Classification
- **Classification applies to titles**: all volumes of a title share the same Dewey code
- **Inheritance**: new volumes automatically inherit title's classification
- **Consistency**: ensures all copies are shelved together

### 13.3 Features
- **Manual assignment**: direct code entry during title creation or editing
- **Google Books Integration**: Automatic population if API returns classification data
- **Display**: Shown in title lists and details

### 13.4 Advantages
- **Simplicity**: Easy to implement and maintain
- **Flexibility**: Users can use any classification scheme they prefer
- **Zero Overhead**: No large reference database required

## 14. Data Validation and Error Handling

### 14.1 Input Validation
- **ISBN validation**: check digit verification for ISBN-10 and ISBN-13 (EAN-13 format)
- **Volume barcode validation**: Code 128 format verification and system-wide uniqueness
- **ISBN barcode validation**: EAN-13 format verification for existing book codes
- **Date validation**: logical date ranges, future date prevention
- **File validation**: image formats (JPEG, PNG, WebP), size limits (5MB)
- **Text sanitization**: prevent XSS, normalize Unicode characters
- **Duplicate detection**: ISBN, volume barcodes, title+author combinations

### 14.2 Error Handling
- **User-friendly messages**: clear error descriptions in user language
- **Error logging**: detailed technical logs for debugging
- **Graceful degradation**: system remains functional during partial failures
- **Retry mechanisms**: automatic retry for transient failures
- **Offline support**: queue operations when network unavailable
- **Data recovery**: automatic recovery from corrupted data

## 15. File and Media Management

### 15.1 Cover Images
- **Supported formats**: JPEG, PNG, WebP
- **Size limits**: maximum 5MB per image
- **Automatic resizing**: generate thumbnails and display sizes
- **Storage**: local filesystem or S3-compatible storage
- **CDN support**: optional content delivery network integration
- **Fallback images**: default covers for missing images

### 15.2 Document Storage
- **File types**: PDF manuals, documentation
- **Version control**: track document updates
- **Access control**: restrict sensitive documents
- **Virus scanning**: automatic malware detection

## 16. Performance and Scalability

### 16.1 Performance Requirements (Personal Use)
- **Response time**: reasonable performance for personal collections (< 2 seconds)
- **Single user focus**: optimized for 1-5 concurrent users
- **Collection size**: efficient handling of personal libraries (up to 10,000 volumes)
- **Simple search**: basic text search without complex indexing

### 16.2 Simple Optimization
- **Basic caching**: browser caching for images and static content
- **Efficient queries**: simple database optimization
- **Progressive loading**: load content as needed for better user experience
- **No complex infrastructure**: designed for simple deployment

## 17. Future Enhancements

### 17.1 Social Features
- **List sharing**: with other users and libraries
- **Cross-recommendations**: between connected libraries
- **Book clubs**: group management and reading lists
- **Reviews and ratings**: user-generated content
- **Reading challenges**: gamification features

### 17.2 Artificial Intelligence
- **Personalized suggestions**: based on reading history and preferences
- **Image recognition**: automatic cover detection and metadata extraction
- **Predictive analysis**: purchase recommendations and trend analysis
- **Natural language search**: semantic search capabilities
- **Auto-categorization**: AI-powered genre and Dewey classification

### 17.3 Advanced Features
- **Mobile app**: native iOS/Android applications
- **Voice commands**: voice-controlled search and operations
- **Augmented reality**: AR-based shelf navigation and book finding
- **Integration with e-readers**: sync reading progress and notes
- **Community features**: local library networks and book exchanges

## 18. Business Rules and Workflows

### 18.0 Advanced Validation Rules

#### 18.0.1 Title Creation Validation
- **Duplicate prevention**: check ISBN and title+author combinations
- **Required fields**: title, at least one author, language
- **ISBN validation**: verify check digits for ISBN-10 and ISBN-13
- **Metadata consistency**: ensure publication year is reasonable
- **Author validation**: verify author exists or create new author record

#### 18.0.2 Volume Addition Validation
- **Barcode uniqueness**: ensure no duplicate barcodes system-wide
- **Title association**: verify volume belongs to existing title
- **Location validation**: ensure location exists in hierarchy
- **Condition validation**: ensure condition is valid enum value
- **Copy number validation**: ensure sequential numbering

#### 18.0.3 Loan Validation Rules
- **Borrower eligibility**: check if borrower can loan (not suspended, under limit)
- **Volume availability**: ensure volume is available for loan
- **Reservation priority**: check if title is reserved by someone else
- **Loan limits**: enforce maximum loans per borrower
- **Condition restrictions**: prevent loaning of damaged volumes

### 18.1 Title Creation Workflow
- **Manual creation**: create title first, then add volumes as needed
- **Import creation**: create title with automatic first volume generation
- **Wishlist creation**: create title without volumes for future acquisition
- **Validation**: prevent duplicate titles (same ISBN or title+author combination)

### 18.2 Volume Management Rules
- **Minimum volumes**: titles can exist with 0 volumes (wishlist)
- **Maximum volumes**: no limit, but warn when exceeding reasonable numbers
- **Copy numbering**: automatic sequential numbering (1, 2, 3...)
- **Deletion rules**: 
  - Cannot delete title if any volume is currently loaned
  - Can delete individual volumes if not loaned
  - Deleting last volume converts title to wishlist status

### 18.3 Loan Business Rules
- **Title-based loans**: users request titles, system assigns best available volume
- **Volume selection priority**: 
  1. Best physical condition
  2. Most accessible location
  3. Lowest copy number (first in, first out)
- **Availability calculation**: title available if at least one volume is available
- **Return processing**: scan any volume to return the associated loan

### 18.4 Data Consistency Rules
- **Referential integrity**: volumes must belong to existing titles
- **Status consistency**: loan status must match volume availability
- **Classification inheritance**: volumes inherit title's Dewey classification
- **Author relationships**: authors linked to titles, not individual volumes
- **Series relationships**: series contain titles, which contain volumes

### 18.5 User Interface Workflows
- **Default view**: show titles with volume counts and availability
- **Expandable details**: click title to see individual volumes
- **Context switching**: toggle between title-centric and volume-centric views
- **Smart actions**: loan/return actions work at title level with volume selection
- **Bulk operations**: select multiple titles for batch operations

### 18.6 Copy Management Interface
- **Add copy button**: prominent button on title detail page
- **Copy management panel**: expandable section showing all volumes
- **Individual copy actions**: edit, delete, move, change condition
- **Bulk copy operations**: select multiple volumes for batch actions
- **Copy status indicators**: visual indicators for availability, condition, location
- **Quick loan interface**: loan specific copy or let system choose

### 18.7 Simple Wishlist System
- **Personal wishlist**: mark titles as "want to read" when all copies are loaned
- **Basic notification**: optional email when title becomes available
- **No complex queues**: simple first-come, first-served for small groups
- **Manual management**: owner can manually notify interested borrowers
- **Integration with wishlist**: titles with 0 volumes can be marked as "interested"

### 18.8 Location Management

#### 18.8.1 Location Hierarchy and Codes
- **Location hierarchy**: building → room → shelf → section
- **Standard codes**: alphanumeric format (e.g., B1-R2-S3-A for Building 1, Room 2, Shelf 3, Section A)
- **Location creation**: interface to add/modify location structure
- **Location validation**: ensure codes follow standard format
- **Location inheritance**: volumes inherit location from parent containers

#### 18.8.2 Location Operations
- **Bulk location updates**: move multiple volumes to new location
- **Location transfer**: move volumes between locations with audit trail
- **Location reports**: inventory by location with volume counts
- **Missing item tracking**: identify volumes not in expected location
- **Location optimization**: suggest optimal placement based on usage patterns

#### 18.8.3 Physical Inventory Management
- **Inventory scanning**: scan locations to verify volume presence
- **Discrepancy reporting**: identify missing or misplaced volumes
- **Location mapping**: visual representation of library layout
- **Capacity management**: track space utilization per location
- **Movement history**: track volume location changes over time

### 18.9 Basic Notifications (Optional)
- **Simple email reminders**: optional overdue notifications
- **Manual notifications**: owner can send custom messages to borrowers
- **No automated systems**: no SMS, push notifications, or complex scheduling
- **Basic alerts**: simple system messages for errors or important events

### 18.10 Simple History Tracking
- **Basic loan history**: track who borrowed what and when
- **Simple change log**: basic record of major modifications
- **No complex auditing**: no detailed logs, IP tracking, or compliance features
- **Manual backup**: simple export/import for data safety

### 18.11 Simple Error Handling
- **Basic validation**: prevent deletion of loaned items with simple confirmation dialogs
- **Simple status tracking**: mark volumes as "lost" or "damaged" without complex workflows
- **No financial management**: no cost calculations, payment tracking, or insurance claims
- **Manual resolution**: owner handles issues personally without automated systems
- **Basic data protection**: simple validation to prevent accidental data loss

### 18.12 Duplicate Management

#### 18.12.1 Automatic Duplicate Detection
- **Title duplicate detection criteria**:
  - Identical ISBN (absolute priority)
  - Exact title + main author match
  - Similar title + identical author (85% fuzzy match)
  - Identical title + similar author (90% fuzzy match)
- **Matching algorithms**:
  - String normalization: remove accents, case, multiple spaces
  - Levenshtein distance for fuzzy matching
  - Tokenization to compare significant keywords
  - Phonetic matching for author names (Soundex/Metaphone)
- **Real-time detection**:
  - Verification during manual title creation
  - Control during data import processes
  - Periodic scan of existing database (weekly)
  - Background processing to avoid performance impact

#### 18.12.2 Duplicate Resolution Interface
- **Duplicate dashboard**:
  - List of potential duplicates with confidence scores
  - Side-by-side preview of suspect titles
  - Filtering by duplicate type (ISBN, title+author, etc.)
  - Batch processing for multiple similar duplicates
- **Merge wizard**:
  - Detailed metadata comparison interface
  - Field selection for preservation (primary vs alternative title)
  - Merge result preview before confirmation
  - Validation step before permanent merge
  - Rollback capability for recent merges

#### 18.12.3 Merge Strategies
- **Title merging process**:
  - Preserve title with most complete metadata
  - Merge all volumes under primary title
  - Renumber volumes sequentially (1, 2, 3...)
  - Transfer active loans to unified title
  - Maintain reservation queue integrity
- **Conflicting metadata handling**:
  - Priority to data with valid ISBN
  - Preserve title variants as alternative titles
  - Merge author lists (main + co-authors)
  - Select best cover image (resolution, quality)
  - Combine subject tags and keywords
- **History preservation**:
  - Maintain loan history from all merged titles
  - Preserve audit trail of all modifications
  - Create links to former identifiers
  - Track merge operations for compliance

#### 18.12.4 Multiple ISBN Management
- **Multiple ISBNs for same title**:
  - Support different editions (hardcover, paperback, pocket)
  - Link ISBNs of different formats for same work
  - Automatic detection of reprints by same publisher
  - Edition grouping under master title
- **ISBN conflict resolution**:
  - Verify check digit validity for all ISBNs
  - External lookup to confirm metadata accuracy
  - Interface to associate/dissociate ISBNs
  - Manual override for special cases
- **Multiple editions handling**:
  - Group editions under master title concept
  - Clear distinction between different editions
  - Separate handling for translations as distinct titles
  - Version tracking (revised, annotated, illustrated)

#### 18.12.5 Duplicate Prevention
- **Creation-time controls**:
  - Automatic search before title creation
  - Suggestions of existing similar titles
  - Mandatory confirmation if potential duplicate detected
  - Smart suggestions based on partial input
- **Secure import process**:
  - Pre-analysis of import files for internal duplicates
  - Duplicate report before import validation
  - Automatic merge option for certain duplicates (identical ISBN)
  - Manual review queue for ambiguous cases
- **Business rules**:
  - Prohibition of creating title with existing ISBN
  - Alert if title+author very similar to existing
  - Mandatory manual validation for ambiguous cases
  - Configurable similarity thresholds

#### 18.12.6 Special Cases
- **Series and volumes**:
  - Distinction between individual volume and complete series
  - Handle different numbering systems (tome 1 vs volume 1)
  - Series-wide duplicate detection
  - Cross-series duplicate identification
- **Reprints and versions**:
  - Differentiate between reprint (same content) and new edition
  - Handle annotated, illustrated, abridged versions
  - Preserve important edition-specific information
  - Track publication history and relationships
- **Translations**:
  - Translations considered as distinct titles
  - Link between original version and translations
  - Handle multiple translation versions
  - Language-specific duplicate detection

#### 18.12.7 Administrative Tools
- **Duplicate reporting**:
  - Export detected duplicates with confidence scores
  - Statistics on most frequent duplicate types
  - Track duplicate resolution over time
  - Performance metrics for detection accuracy
- **Cleanup tools**:
  - Batch merge for certain duplicates (same ISBN)
  - Remove empty titles after merging
  - Automatic volume renumbering
  - Orphaned record cleanup
- **Audit and traceability**:
  - Complete log of all merge operations
  - Ability to undo merge if error detected
  - History of non-merge decisions
  - Compliance reporting for data integrity

#### 18.12.8 API and Integrations
- **Dedicated endpoints**:
  - `GET /api/v1/duplicates` - List potential duplicates with pagination
  - `POST /api/v1/duplicates/merge` - Merge two titles with validation
  - `POST /api/v1/duplicates/ignore` - Mark as non-duplicate permanently
  - `GET /api/v1/duplicates/suggestions/{title-id}` - Get suggestions for specific title
  - `PUT /api/v1/duplicates/confidence/{id}` - Update confidence score manually
- **External integration**:
  - Verification with external bibliographic databases
  - Cross-validation with Google Books, WorldCat, OpenLibrary
  - Import enriched metadata to resolve ambiguities
  - API rate limiting and caching for external services
  - Fallback mechanisms when external services unavailable

## 19. Mobile and Responsive Design

### 19.1 Mobile-First Interface
- **Touch-optimized**: large buttons, swipe gestures, tap targets
- **Responsive layout**: adaptive design for all screen sizes
- **Offline capability**: core functions available without internet
- **Progressive web app**: installable, app-like experience
- **Performance optimization**: fast loading on mobile networks

### 19.2 Mobile-Specific Features
- **Camera integration**: scan barcodes using device camera
- **Location services**: find nearby volumes, optimize shelf navigation
- **Push notifications**: real-time alerts and reminders
- **Voice search**: hands-free search and navigation
- **Gesture controls**: swipe to loan/return, pinch to zoom

### 19.3 Tablet Optimization
- **Split-screen views**: show title list and details simultaneously
- **Drag-and-drop**: move volumes between locations visually
- **Multi-touch**: select multiple items with touch gestures
- **Landscape mode**: optimized layouts for horizontal orientation
- **Stylus support**: precise input for detailed operations

## Development Priorities (Personal Use)

### Phase 1 (MVP)
- Basic title and volume management
- Simple loan tracking (who borrowed what, when)
- Web interface with barcode scanner support
- Basic search and browsing
- Manual backup/restore (export/import)

### Phase 2
- Author and series management
- Multiple copies per title
- Wishlist for titles without volumes
- Basic statistics and reports
- Volume condition tracking

### Phase 3
- Optional Dewey classification
- Google Books integration for metadata
- Basic email notifications (optional)
- Simple mobile-responsive interface
- Location management for physical organization

### Phase 4 (Optional Enhancements)
- French/English interface toggle
- Advanced search and filtering
- Simple sharing features for small groups
- Enhanced import/export formats
- Basic duplicate detection
