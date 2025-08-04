# Incremental Development Planning - Library Management System

## Overview
This planning breaks down development into very small steps, each bringing a testable and usable functionality with Leptos as the frontend framework. Each step can be developed, tested and deployed independently.

## Phase 1: Foundations (Minimal MVP)

### Step 1.1: Initial project setup (1-2 days)
**Objective**: Have a compilable Rust project with basic structure
- [x] Create backend/frontend folder structure
- [x] Configure Cargo.toml for backend (Axum + SQLx + MariaDB)
- [x] Configure Cargo.toml for frontend (Leptos + reqwest-wasm)
- [x] Configure Trunk.toml for WASM build
- [x] Create "Hello World" in Axum (backend)
- [x] Create "Hello World" in Leptos (frontend)
- [x] Verify everything compiles and runs

**Deliverable**: Leptos application displaying "Hello World" on http://localhost:3000

### Step 1.2: Simple MariaDB database (1 day)
**Objective**: Functional MariaDB connection
- [ ] Configure SQLx with MariaDB (MySQL driver)
- [ ] Create basic database configuration
- [ ] Test MariaDB connection
- [ ] Create `/health` endpoint that checks DB

**Deliverable**: `/api/health` endpoint returning DB status

### Step 1.3: First "volumes" table MariaDB (1 day)
**Objective**: Be able to store basic volumes
- [ ] Create MariaDB migration for `volumes` table (minimal version)
- [ ] Define `Volume` model with essential fields only
- [ ] Create `VolumeRepository` with `create` method for MariaDB
- [ ] Test hardcoded volume insertion

**Deliverable**: Volumes table created in MariaDB, insertion possible via code

### Step 1.4: API POST /volumes (1 day)
**Objective**: Create a volume via API
- [ ] Create `create_volume` handler
- [ ] Define `CreateVolumeRequest` (title + ISBN only)
- [ ] Implement basic validation with validator
- [ ] Test with curl/Postman

**Deliverable**: `POST /api/v1/volumes` functional

### Step 1.5: API GET /volumes (1 day)
**Objective**: List created volumes
- [ ] Implement `find_all` in repository
- [ ] Create `get_volumes` handler
- [ ] Return JSON list of volumes
- [ ] Test retrieval

**Deliverable**: `GET /api/v1/volumes` returns list of volumes

### Step 1.6: Leptos Frontend - API Service (1 day)
**Objective**: Frontend-backend communication
- [ ] Create `ApiClient` service with reqwest-wasm
- [ ] Implement `get_volumes()` and `create_volume()`
- [ ] Handle HTTP errors and JSON serialization
- [ ] Test API calls from frontend

**Deliverable**: Functional API service on frontend side

### Step 1.7: Leptos Frontend - Volume display (1-2 days)
**Objective**: See volumes in browser
- [ ] Create `VolumesList` component with create_resource
- [ ] Implement basic `VolumeCard` component
- [ ] Handle loading states with Suspense
- [ ] Handle errors with ErrorBoundary

**Example code**:
```rust
#[component]
pub fn VolumesList() -> impl IntoView {
    let volumes = create_resource(|| (), |_| async { 
        api_client().get_volumes().await 
    });

    view! {
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            {move || volumes.get().map(|vols| match vols {
                Ok(volumes) => volumes.into_iter()
                    .map(|vol| view! { <VolumeCard volume=vol/> })
                    .collect_view(),
                Err(_) => view! { <p>"Loading error"</p> }.into_view()
            })}
        </Suspense>
    }
}
```

**Deliverable**: Leptos web page displaying list of volumes

### Step 1.8: Leptos Frontend - Add form (1-2 days)
**Objective**: Add a volume from interface
- [ ] Create `VolumeForm` component with reactive signals
- [ ] Use create_action for submission
- [ ] Client-side validation with same rules as backend
- [ ] Automatically refresh list after addition

**Example code**:
```rust
#[component]
pub fn VolumeForm() -> impl IntoView {
    let (title, set_title) = create_signal(String::new());
    let (isbn, set_isbn) = create_signal(String::new());
    
    let submit_action = create_action(|data: &CreateVolumeRequest| {
        let data = data.clone();
        async move { api_client().create_volume(data).await }
    });

    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            submit_action.dispatch(CreateVolumeRequest {
                title: title.get(),
                isbn: isbn.get(),
            });
        }>
            <input 
                prop:value=title
                on:input=move |ev| set_title(event_target_value(&ev))
                placeholder="Title"
            />
            <input 
                prop:value=isbn
                on:input=move |ev| set_isbn(event_target_value(&ev))
                placeholder="ISBN"
            />
            <button type="submit">"Add"</button>
        </form>
    }
}
```

**Deliverable**: Functional Leptos form for adding volumes

## Phase 2: Basic Features

### Step 2.1: Barcode generation (1 day)
**Objective**: Each volume has a unique barcode
- [ ] Create `BarcodeGenerator` service
- [ ] Implement sequential generation (BIB-000001)
- [ ] Modify volume creation to generate code
- [ ] Display barcode in `VolumeCard`

**Deliverable**: Volumes created with automatic barcodes

### Step 2.2: Barcode search (1 day)
**Objective**: Find a volume by its code
- [ ] Implement `find_by_barcode` in repository
- [ ] Create endpoint `GET /api/v1/scan/{barcode}`
- [ ] Add `scan_barcode()` to frontend API service
- [ ] Test barcode search

**Deliverable**: Functional barcode search API

### Step 2.3: Leptos scan page (1-2 days)
**Objective**: Interface to scan/enter a barcode
- [ ] Create `ScanPage` with Leptos routing
- [ ] Reactive input field with validation
- [ ] Use create_action for search
- [ ] Conditional display of found volume
- [ ] Elegant handling of "volume not found" case

**Example code**:
```rust
#[component]
pub fn ScanPage() -> impl IntoView {
    let (barcode, set_barcode) = create_signal(String::new());
    let scan_action = create_action(|barcode: &String| {
        let barcode = barcode.clone();
        async move { api_client().scan_barcode(&barcode).await }
    });

    view! {
        <div class="max-w-2xl mx-auto">
            <h1>"Scan a barcode"</h1>
            <input 
                prop:value=barcode
                on:input=move |ev| set_barcode(event_target_value(&ev))
                on:keydown=move |ev| {
                    if ev.key() == "Enter" && !barcode.get().is_empty() {
                        scan_action.dispatch(barcode.get());
                    }
                }
                placeholder="Scan or enter barcode..."
                autofocus
            />
            
            <Show when=move || scan_action.pending().get()>
                <p>"Searching..."</p>
            </Show>
            
            {move || scan_action.value().get().map(|result| match result {
                Ok(Some(volume)) => view! {
                    <div class="mt-4 p-4 bg-green-50 border border-green-200 rounded">
                        <h3>"Volume found!"</h3>
                        <VolumeCard volume=volume />
                    </div>
                }.into_view(),
                Ok(None) => view! {
                    <p class="text-orange-600">"No volume found"</p>
                }.into_view(),
                Err(_) => view! {
                    <p class="text-red-600">"Search error"</p>
                }.into_view(),
            })}
        </div>
    }
}
```

**Deliverable**: Functional Leptos scan page

### Step 2.4: Borrowers table MariaDB (1 day)
**Objective**: Be able to register borrowers
- [ ] Create MariaDB migration for `borrowers` table
- [ ] Basic `Borrower` model (name + email)
- [ ] `BorrowerRepository` optimized for MariaDB
- [ ] Basic CRUD API for borrowers

**Deliverable**: Basic borrower management in MariaDB

### Step 2.5: Loans table MariaDB (1 day)
**Objective**: Record loans
- [ ] Create MariaDB migration for `loans` table
- [ ] `Loan` model (volume_id, borrower_id, dates)
- [ ] `LoanRepository` with optimized MariaDB queries
- [ ] Database constraints and MariaDB indexes

**Deliverable**: Data structure for loans in MariaDB

### Step 2.6: Simple loan API (1-2 days)
**Objective**: Loan a volume
- [ ] Endpoint `POST /api/v1/loans`
- [ ] Check that volume is available
- [ ] Create loan with due date
- [ ] Mark volume as loaned

**Deliverable**: Functional loan API

### Step 2.7: Return API (1 day)
**Objective**: Return a loaned volume
- [ ] Endpoint `PUT /api/v1/loans/{id}/return`
- [ ] Mark loan as returned
- [ ] Free the volume

**Deliverable**: Functional return API

### Step 2.8: Leptos loan/return interface (2 days)
**Objective**: Loan/return from interface
- [ ] `BorrowerSelector` component with reactive search
- [ ] Add loan/return buttons on scan page
- [ ] Leptos modal for borrower selection
- [ ] Reactive actions for loan/return
- [ ] Toast notifications with signals

**Example code**:
```rust
#[component]
pub fn LoanActions(volume: Volume) -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);
    let loan_action = create_action(|req: &LoanRequest| {
        let req = req.clone();
        async move { api_client().create_loan(req).await }
    });

    view! {
        <div class="flex space-x-2">
            <button 
                class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
                on:click=move |_| set_show_modal(true)
            >
                "Loan"
            </button>
            
            <Show when=show_modal>
                <BorrowerModal 
                    volume=volume.clone()
                    on_close=move || set_show_modal(false)
                    on_loan=move |borrower_id| {
                        loan_action.dispatch(LoanRequest {
                            volume_id: volume.id,
                            borrower_id,
                        });
                        set_show_modal(false);
                    }
                />
            </Show>
        </div>
    }
}
```

**Deliverable**: Complete Leptos loan/return interface

## Phase 3: Experience Enhancement

### Step 3.1: Volume states MariaDB (1 day)
**Objective**: Manage physical state of volumes
- [ ] Add `condition` ENUM field to MariaDB volumes table
- [ ] Rust enum for states (excellent, good, fair, poor, lost)
- [ ] Leptos `ConditionSelector` component
- [ ] Prevent loaning volumes in poor condition

**Deliverable**: Volume state management with MariaDB ENUM

### Step 3.2: Extended volume information MariaDB (1-2 days)
**Objective**: More metadata
- [ ] Add MariaDB fields: publisher, year, pages, genre
- [ ] Modify MariaDB migrations and Rust models
- [ ] Extended `VolumeForm` component with reactive validation
- [ ] Improve `VolumeCard` with all info
- [ ] Optimize MariaDB indexes for new columns

**Deliverable**: Volumes with complete metadata in MariaDB

### Step 3.3: Physical location (1 day)
**Objective**: Know where volumes are stored
- [ ] Add `location` field
- [ ] `LocationInput` component with suggestions
- [ ] Display location in lists
- [ ] Filter by location

**Deliverable**: Physical location management

### Step 3.4: Authors management MariaDB (2 days)
**Objective**: Associate authors with volumes
- [ ] Create MariaDB `authors` table with UTF8MB4 encoding
- [ ] Junction table `volume_authors` with MariaDB foreign keys
- [ ] CRUD API for authors with optimized MariaDB queries
- [ ] Leptos `AuthorsList` and `AuthorForm` components

**Deliverable**: Basic author management in MariaDB

### Step 3.5: Volume-author association Leptos (1-2 days)
**Objective**: Link volumes and authors
- [ ] `AuthorSelector` component with real-time search
- [ ] Modify `VolumeForm` to select authors
- [ ] Display authors in `VolumeCard`
- [ ] Author detail page with their volumes

**Example code**:
```rust
#[component]
pub fn AuthorSelector(selected_authors: RwSignal<Vec<Author>>) -> impl IntoView {
    let (search, set_search) = create_signal(String::new());
    let authors = create_resource(
        move || search.get(),
        |search_term| async move {
            if search_term.len() >= 2 {
                api_client().search_authors(&search_term).await
            } else {
                Ok(vec![])
            }
        }
    );

    view! {
        <div class="relative">
            <input 
                prop:value=search
                on:input=move |ev| set_search(event_target_value(&ev))
                placeholder="Search an author..."
            />
            
            <Show when=move || !authors.get().unwrap_or_default().is_empty()>
                <div class="absolute z-10 w-full bg-white border rounded-md shadow-lg">
                    <For
                        each=move || authors.get().unwrap_or_default()
                        key=|author| author.id
                        children=move |author| {
                            view! {
                                <button 
                                    class="w-full px-4 py-2 text-left hover:bg-gray-100"
                                    on:click=move |_| {
                                        selected_authors.update(|authors| authors.push(author.clone()));
                                        set_search(String::new());
                                    }
                                >
                                    {format!("{} {}", author.first_name.unwrap_or_default(), author.last_name)}
                                </button>
                            }
                        }
                    />
                </div>
            </Show>
        </div>
    }
}
```

**Deliverable**: Functional volume-author linking

## Phase 4: Advanced Features

### Step 4.1: MariaDB + Leptos text search (1-2 days)
**Objective**: Search in titles and descriptions
- [ ] Add FULLTEXT MariaDB indexes for search
- [ ] Search endpoint with MATCH AGAINST MariaDB queries
- [ ] `SearchBar` component with debouncing
- [ ] Reactive filters by availability, state, etc.

**Example code**:
```rust
#[component]
pub fn SearchBar() -> impl IntoView {
    let (search_term, set_search_term) = create_signal(String::new());
    let (filters, set_filters) = create_signal(SearchFilters::default());
    
    // Debounce search to avoid too many API calls
    let debounced_search = create_memo(move |_| {
        let term = search_term.get();
        if term.len() >= 2 { Some(term) } else { None }
    });
    
    let search_results = create_resource(
        move || (debounced_search.get(), filters.get()),
        |(term, filters)| async move {
            match term {
                Some(term) => api_client().search_volumes(&term, &filters).await,
                None => Ok(vec![])
            }
        }
    );

    view! {
        <div class="space-y-4">
            <input 
                prop:value=search_term
                on:input=move |ev| set_search_term(event_target_value(&ev))
                placeholder="Search volumes..."
                class="w-full px-4 py-2 border rounded-md"
            />
            
            <SearchFilters filters=filters set_filters=set_filters />
            
            <Suspense fallback=move || view! { <p>"Searching..."</p> }>
                <SearchResults results=search_results />
            </Suspense>
        </div>
    }
}
```

**Deliverable**: Functional Leptos advanced search

### Step 4.2: Dewey classification MariaDB (2 days)
**Objective**: Classify volumes according to Dewey
- [ ] Add `dewey_code` and `dewey_category` fields in MariaDB
- [ ] MariaDB reference table for Dewey codes with indexes
- [ ] `DeweySelector` component with hierarchical navigation
- [ ] Navigation by classification with optimized MariaDB queries

**Deliverable**: Basic Dewey classification in MariaDB

### Step 4.3: Series management MariaDB + Leptos (2-3 days)
**Objective**: Organize volumes in series
- [ ] Create MariaDB `series` table with constraints
- [ ] Associate volumes to series with MariaDB junction table
- [ ] `SeriesManager` component with drag & drop for ordering
- [ ] Series management interface with optimized MariaDB queries

**Deliverable**: Functional series management in MariaDB

### Step 4.4: Dashboard with MariaDB statistics (1-2 days)
**Objective**: Dashboard with metrics
- [ ] Endpoint `/api/v1/stats/dashboard` with aggregated MariaDB queries
- [ ] Optimized MariaDB calculations: COUNT, SUM, GROUP BY for statistics
- [ ] Leptos `StatCard` and `Chart` components
- [ ] Real-time updates with signals and MariaDB cache

**Example code**:
```rust
#[component]
pub fn Dashboard() -> impl IntoView {
    let stats = create_resource(|| (), |_| async {
        api_client().get_dashboard_stats().await
    });

    view! {
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
            <Suspense fallback=move || view! { <div class="animate-pulse bg-gray-200 h-24 rounded"></div> }>
                {move || stats.get().map(|stats| match stats {
                    Ok(stats) => view! {
                        <StatCard 
                            title="Total Volumes"
                            value=stats.total_volumes
                            icon="📚"
                        />
                        <StatCard 
                            title="Active Loans"
                            value=stats.active_loans
                            icon="📖"
                        />
                        <StatCard 
                            title="Overdue Volumes"
                            value=stats.overdue_loans
                            icon="⚠️"
                        />
                    }.into_view(),
                    Err(_) => view! { <p>"Loading error"</p> }.into_view()
                })}
            </Suspense>
        </div>
    }
}
```

**Deliverable**: Leptos dashboard with statistics

### Step 4.5: Overdue management (1-2 days)
**Objective**: Identify and manage overdue loans
- [ ] Automatic overdue calculation
- [ ] Endpoint `/api/v1/loans/overdue`
- [ ] `OverdueLoans` component with actions
- [ ] Reactive visual notifications

**Deliverable**: Overdue loan management

## Phase 5: Multi-DBMS support and internationalization

### Step 5.1: Multi-DBMS database abstraction (2-3 days)
**Objective**: Support MariaDB (default), PostgreSQL, MySQL
- [ ] Create `DatabaseFactory` with MariaDB as default implementation
- [ ] Separate migrations: MariaDB (main), PostgreSQL, MySQL
- [ ] Dynamic DB type configuration with MariaDB as default
- [ ] Tests with all 3 DBMS, focus on MariaDB
- [ ] Documentation of differences between DBMS

**Deliverable**: Multi-DBMS support with optimized MariaDB by default

### Step 5.2: Backend internationalization (2 days)
**Objective**: Multilingual error messages
- [ ] `I18nService` service
- [ ] Translation files (FR, EN)
- [ ] Language detection middleware
- [ ] Translated API messages

**Deliverable**: Multilingual backend

### Step 5.3: Leptos internationalization (2-3 days)
**Objective**: Multilingual user interface
- [ ] leptos-i18n integration
- [ ] `LanguageSelector` component
- [ ] Translation of all texts with reactive signals
- [ ] Localized date formats

**Example code**:
```rust
#[component]
pub fn LanguageSelector() -> impl IntoView {
    let i18n = use_i18n();
    let (current_locale, set_locale) = create_signal(i18n.get_locale());

    view! {
        <select 
            prop:value=move || current_locale.get().to_string()
            on:change=move |ev| {
                let new_locale = event_target_value(&ev);
                i18n.set_locale(new_locale.parse().unwrap());
                set_locale(new_locale.parse().unwrap());
            }
        >
            <option value="fr">{t!(i18n, "language.french")}</option>
            <option value="en">{t!(i18n, "language.english")}</option>
            <option value="es">{t!(i18n, "language.spanish")}</option>
        </select>
    }
}
```

**Deliverable**: Complete multilingual Leptos frontend

## Phase 6: Advanced Features

### Step 6.1: CSV Import/Export (2 days)
**Objective**: Import/export data
- [ ] CSV export endpoint
- [ ] CSV import endpoint with validation
- [ ] Leptos `ImportExport` component with file upload
- [ ] Import error handling with feedback

**Deliverable**: Data import/export

### Step 6.2: Label generation (2-3 days)
**Objective**: Print labels with barcodes
- [ ] Barcode image generation
- [ ] Label templates (Avery, etc.)
- [ ] PDF generation endpoint
- [ ] Leptos selection and preview interface

**Deliverable**: Printable label generation

### Step 6.3: External API integration (2-3 days)
**Objective**: Retrieve metadata via ISBN
- [ ] Google Books API client
- [ ] Automatic metadata retrieval
- [ ] Cover download
- [ ] Leptos `ISBNImport` component with preview

**Deliverable**: Automatic data enrichment

### Step 6.4: Notifications and reminders (2 days)
**Objective**: Loan alerts
- [ ] Notification service
- [ ] Due date calculation
- [ ] Leptos `NotificationCenter` component
- [ ] Reactive toast notifications

**Deliverable**: Notification system

## Phase 7: Optimization and deployment

### Step 7.1: Redis cache (1-2 days)
**Objective**: Improve performance
- [ ] Redis integration
- [ ] Cache frequent queries
- [ ] Smart invalidation
- [ ] Cache metrics

**Deliverable**: Performant cache system

### Step 7.2: Automated tests with MariaDB (2-3 days)
**Objective**: Complete test coverage
- [ ] Unit tests for all services
- [ ] API integration tests with MariaDB test database
- [ ] Leptos tests with wasm-bindgen-test
- [ ] CI/CD with GitHub Actions and MariaDB container
- [ ] MariaDB-specific performance tests

**Deliverable**: Complete test suite with MariaDB

### Step 7.3: Containerization with MariaDB (1-2 days)
**Objective**: Docker deployment
- [ ] Optimized Dockerfiles (multi-stage for Leptos)
- [ ] Docker Compose with MariaDB by default, multi-DBMS support
- [ ] Optimized MariaDB deployment scripts
- [ ] Deployment documentation with MariaDB focus
- [ ] MariaDB configuration for production

**Deliverable**: Containerized deployment with optimized MariaDB

### Step 7.4: Monitoring and logs (1-2 days)
**Objective**: Production observability
- [ ] Structured logs with tracing
- [ ] Performance metrics
- [ ] Health checks
- [ ] Monitoring alerts

**Deliverable**: Complete monitoring

## Technical Specifications

### Leptos advantages for this project:
- **Fine reactivity**: Volume lists update automatically
- **Type safety**: Type sharing between frontend and backend
- **Performance**: Optimized WASM compilation
- **Reusable components**: `VolumeCard`, `SearchBar`, etc.

### MariaDB advantages for this project:
- **Performance**: Optimized queries for text searches
- **Compatibility**: 100% MySQL compatible with more features
- **FULLTEXT**: Native performant text search
- **JSON**: Native JSON column support for metadata
- **Open Source**: Completely free and community-driven

### Recommended tools:
- **Trunk**: Build tool for WASM
- **leptos-i18n**: Internationalization
- **reqwest-wasm**: HTTP client
- **web-sys**: DOM interactions
- **wasm-bindgen**: JavaScript bindings
- **MariaDB Connector**: Via SQLx with MariaDB optimizations

## Total Estimation
- **Phase 1**: 8-12 days (Functional Leptos MVP)
- **Phase 2**: 10-14 days (Basic features)
- **Phase 3**: 8-12 days (UX improvement)
- **Phase 4**: 10-15 days (Advanced features)
- **Phase 5**: 6-8 days (Multi-DBMS + i18n)
- **Phase 6**: 8-12 days (Advanced features)
- **Phase 7**: 6-9 days (Optimization)

**Total estimated**: 56-82 development days

## Important Notes
- Each step must be tested before moving to the next
- Estimates include testing and documentation
- Leptos allows faster development thanks to reactivity
- Allow time for learning Leptos if necessary
- Each step produces a deployable and usable version
- Leptos reactivity simplifies complex state management
