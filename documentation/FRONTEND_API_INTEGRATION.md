# Frontend API Integration Guide

This document explains how the frontend fetches and displays titles from the backend API.

## Architecture Overview

```
┌─────────────────┐     HTTP GET      ┌──────────────────┐
│  Slint UI       │ ←──────────────── │  API Client      │
│  (TitlesPage)   │                   │  (api_client.rs) │
└─────────────────┘                   └──────────────────┘
         ↑                                      ↓
         │                                      │ HTTP
         │ Callbacks                            │
         │                                      ↓
┌─────────────────┐                   ┌──────────────────┐
│  main.rs        │                   │  Backend API     │
│  (Rust logic)   │                   │  localhost:8000  │
└─────────────────┘                   └──────────────────┘
```

## Components

### 1. Models (`frontend/src/models.rs`)

Rust structs that match the backend API response:

```rust
pub struct Title {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub isbn: Option<String>,
    // ... other fields
}

pub struct TitleWithCount {
    pub title: Title,
    pub volume_count: i64,
}
```

### 2. API Client (`frontend/src/api_client.rs`)

HTTP client using `reqwest` (blocking mode for native desktop):

```rust
pub struct ApiClient {
    base_url: String,
    client: reqwest::blocking::Client,
}

impl ApiClient {
    pub fn get_titles(&self) -> Result<Vec<TitleWithCount>, Box<dyn Error>> {
        let url = format!("{}/api/v1/titles", self.base_url);
        let response = self.client.get(&url).send()?;
        let titles: Vec<TitleWithCount> = response.json()?;
        Ok(titles)
    }
}
```

### 3. Slint UI (`frontend/ui/pages/titles_page.slint`)

UI component that displays the titles:

- **TitleData struct**: Slint-compatible data structure
- **TitlesPage component**: Main page component with:
  - `titles` property: Array of title data
  - `load-titles` callback: Triggered when refresh is needed
  - ListView: Displays titles in a scrollable list

### 4. Main Application (`frontend/src/main.rs`)

Connects everything together:

1. Creates API client
2. Defines `load_titles` function that:
   - Fetches titles from backend
   - Converts to Slint data format
   - Updates UI
3. Connects callback to UI
4. Loads titles on startup

## How to Use

### Running the Application

1. **Start the backend:**
   ```bash
   cd backend
   cargo run
   ```
   Backend runs on `http://localhost:8000`

2. **Start the frontend:**
   ```bash
   cd frontend
   cargo run
   ```

### What Happens

1. Frontend starts and automatically fetches titles
2. Console shows:
   ```
   Loading titles from backend...
   Fetching titles from: http://localhost:8000/api/v1/titles
   Successfully fetched 5 titles
   UI updated with titles
   ```

3. UI displays titles in the first tab ("Volumes")
4. Click "Refresh" button to reload titles

### Navigation

- **Volumes tab** (first item): Shows list of titles with volume counts
- **About tab** (last item): About page

## Data Flow

### On Startup:
```
main()
  → load_titles()
    → api_client.get_titles()
      → HTTP GET /api/v1/titles
      → Backend returns JSON
    → Convert to Slint TitleData
    → ui.set_titles(model)
  → UI displays titles
```

### On Refresh Button Click:
```
User clicks "Refresh"
  → TitlesPage.load-titles callback
    → load_titles() in main.rs
      → (same flow as startup)
```

## Configuration

### Changing Backend URL

Edit `api_client.rs`:

```rust
impl Default for ApiClient {
    fn default() -> Self {
        Self::new("http://your-server:port".to_string())
    }
}
```

Or create custom client:

```rust
let api_client = ApiClient::new("http://example.com:8080".to_string());
```

## Error Handling

If backend is not running:

```
Failed to fetch titles: error sending request for url (http://localhost:8000/api/v1/titles): error trying to connect: tcp connect error: Connection refused (os error 111)
Make sure the backend server is running on http://localhost:8000
```

The UI will show: "No titles found. Click Refresh to load titles from the backend."

## Adding More API Calls

To add more endpoints:

1. **Add method to ApiClient:**
   ```rust
   pub fn get_volumes(&self) -> Result<Vec<Volume>, Box<dyn Error>> {
       let url = format!("{}/api/v1/volumes", self.base_url);
       // ...
   }
   ```

2. **Create Slint struct and page** (similar to TitlesPage)

3. **Connect in main.rs** (similar to load_titles)

## Dependencies

Frontend dependencies in `Cargo.toml`:

```toml
[dependencies]
slint = "1.14.1"
reqwest = { version = "0.12", features = ["blocking", "json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```

- **slint**: UI framework
- **reqwest**: HTTP client (blocking mode for desktop)
- **serde/serde_json**: JSON serialization/deserialization
- **chrono**: Date/time handling

## Future: WASM Support

When switching to WASM:

1. Change `reqwest` features:
   ```toml
   reqwest = { version = "0.12", features = ["json"], default-features = false }
   ```

2. Use async API client instead of blocking

3. Add WASM-specific bindings for HTTP

The UI code (Slint) remains the same!

## Troubleshooting

### "No titles found" but backend is running

Check:
1. Backend is on `http://localhost:8000`
2. Check browser/logs: `curl http://localhost:8000/api/v1/titles`
3. Look at console output for error messages

### Compilation errors

```bash
cd frontend
cargo clean
cargo build
```

### Backend connection refused

Make sure backend is running:
```bash
cd backend
cargo run
```

Check it responds:
```bash
curl http://localhost:8000/health
curl http://localhost:8000/api/v1/titles
```
