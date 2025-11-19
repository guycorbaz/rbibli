use crate::models::{
    TitleWithCount, CreateTitleRequest, UpdateTitleRequest, LocationWithPath, CreateLocationRequest,
    UpdateLocationRequest, AuthorWithTitleCount, CreateAuthorRequest, UpdateAuthorRequest,
    PublisherWithTitleCount, CreatePublisherRequest, UpdatePublisherRequest, GenreWithTitleCount,
    CreateGenreRequest, UpdateGenreRequest, SeriesWithTitleCount, CreateSeriesRequest, UpdateSeriesRequest,
    Volume, CreateVolumeRequest, UpdateVolumeRequest,
    IsbnLookupResponse, BorrowerGroup, CreateBorrowerGroupRequest, UpdateBorrowerGroupRequest,
    BorrowerWithGroup, CreateBorrowerRequest, UpdateBorrowerRequest,
    LoanDetail, CreateLoanRequest, CreateLoanResponse,
    DeweySearchResult,
    LibraryStatistics, GenreStatistic, LocationStatistic, LoanStatistic
};
use std::error::Error;

/// API client for communicating with the rbibli backend
pub struct ApiClient {
    base_url: String,
    client: reqwest::blocking::Client,
}

impl ApiClient {
    /// Creates a new API client with the specified base URL.
    ///
    /// This constructor initializes a new `ApiClient` instance configured to communicate
    /// with a backend server at the given base URL. It creates a blocking HTTP client
    /// using `reqwest` for making synchronous API requests.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the backend API server (e.g., "http://localhost:8000")
    ///
    /// # Returns
    ///
    /// A new `ApiClient` instance ready to make API requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbibli_frontend::api_client::ApiClient;
    ///
    /// // Create a client for a local development server
    /// let client = ApiClient::new("http://localhost:8000".to_string());
    ///
    /// // Create a client for a remote server
    /// let remote_client = ApiClient::new("https://api.example.com".to_string());
    /// ```
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::blocking::Client::new(),
        }
    }

    /// Fetches all titles with their volume counts from the backend API.
    ///
    /// This method makes a GET request to the `/api/v1/titles` endpoint to retrieve
    /// a list of all titles in the library. Each title includes its metadata along
    /// with a count of how many physical volumes (copies) exist for that title.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<TitleWithCount>)` - A vector of titles with volume counts on success
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The HTTP request fails (network error, timeout, etc.)
    ///   - The server returns a non-success status code
    ///   - The response body cannot be parsed as JSON
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    ///
    /// let client = ApiClient::default();
    /// match client.get_titles() {
    ///     Ok(titles) => println!("Found {} titles", titles.len()),
    ///     Err(e) => eprintln!("Failed to fetch titles: {}", e),
    /// }
    /// ```
    pub fn get_titles(&self) -> Result<Vec<TitleWithCount>, Box<dyn Error>> {
        let url = format!("{}/api/v1/titles", self.base_url);

        println!("Fetching titles from: {}", url);

        let response = self.client.get(&url).send()?;

        if !response.status().is_success() {
            return Err(format!("API returned status: {}", response.status()).into());
        }

        let titles: Vec<TitleWithCount> = response.json()?;

        println!("Successfully fetched {} titles", titles.len());

        Ok(titles)
    }

    /// Searches titles with advanced filtering options.
    ///
    /// This method makes a GET request to the `/api/v1/titles/search` endpoint with
    /// query parameters for filtering and searching titles. All parameters are optional.
    ///
    /// # Arguments
    ///
    /// * `search_text` - Free text search across title, subtitle, author, ISBN (optional)
    /// * `title` - Partial match on title field (optional)
    /// * `subtitle` - Partial match on subtitle field (optional)
    /// * `isbn` - Partial or exact match on ISBN (optional)
    /// * `series_id` - Filter by series UUID (optional)
    /// * `author_id` - Filter by author UUID (optional)
    /// * `genre_id` - Filter by genre UUID (optional)
    /// * `publisher_id` - Filter by publisher UUID (optional)
    /// * `year_from` - Minimum publication year (optional)
    /// * `year_to` - Maximum publication year (optional)
    /// * `language` - Filter by language code (optional)
    /// * `dewey_code` - Filter by Dewey classification (optional)
    /// * `has_volumes` - Filter for owned books (true), wishlist (false), or all (None)
    /// * `available_only` - Filter for books with available volumes (optional)
    /// * `location_id` - Filter by storage location UUID (optional)
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<TitleWithCount>)` - A vector of matching titles with volume counts
    /// * `Err(Box<dyn Error>)` - An error if the request fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    ///
    /// let client = ApiClient::default();
    ///
    /// // Simple text search
    /// match client.search_titles(Some("harry potter"), None, None, None, None, None, None, None, None, None, None, None, None, None, None) {
    ///     Ok(titles) => println!("Found {} titles", titles.len()),
    ///     Err(e) => eprintln!("Search failed: {}", e),
    /// }
    ///
    /// // Search by genre and year range
    /// match client.search_titles(None, None, None, None, None, None, Some("genre-uuid"), None, Some("2000"), Some("2023"), None, None, None, None, None) {
    ///     Ok(titles) => println!("Found {} titles", titles.len()),
    ///     Err(e) => eprintln!("Search failed: {}", e),
    /// }
    /// ```
    pub fn search_titles(
        &self,
        search_text: Option<&str>,
        title: Option<&str>,
        subtitle: Option<&str>,
        isbn: Option<&str>,
        series_id: Option<&str>,
        author_id: Option<&str>,
        genre_id: Option<&str>,
        publisher_id: Option<&str>,
        year_from: Option<&str>,
        year_to: Option<&str>,
        language: Option<&str>,
        dewey_code: Option<&str>,
        has_volumes: Option<bool>,
        available_only: Option<bool>,
        location_id: Option<&str>,
    ) -> Result<Vec<TitleWithCount>, Box<dyn Error>> {
        let mut url = format!("{}/api/v1/titles/search?", self.base_url);
        let mut params = Vec::new();

        // Build query parameters
        if let Some(q) = search_text {
            if !q.is_empty() {
                params.push(format!("q={}", urlencoding::encode(q)));
            }
        }
        if let Some(t) = title {
            if !t.is_empty() {
                params.push(format!("title={}", urlencoding::encode(t)));
            }
        }
        if let Some(s) = subtitle {
            if !s.is_empty() {
                params.push(format!("subtitle={}", urlencoding::encode(s)));
            }
        }
        if let Some(i) = isbn {
            if !i.is_empty() {
                params.push(format!("isbn={}", urlencoding::encode(i)));
            }
        }
        if let Some(sid) = series_id {
            if !sid.is_empty() {
                params.push(format!("series_id={}", urlencoding::encode(sid)));
            }
        }
        if let Some(aid) = author_id {
            if !aid.is_empty() {
                params.push(format!("author_id={}", urlencoding::encode(aid)));
            }
        }
        if let Some(gid) = genre_id {
            if !gid.is_empty() {
                params.push(format!("genre_id={}", urlencoding::encode(gid)));
            }
        }
        if let Some(pid) = publisher_id {
            if !pid.is_empty() {
                params.push(format!("publisher_id={}", urlencoding::encode(pid)));
            }
        }
        if let Some(yf) = year_from {
            if !yf.is_empty() {
                params.push(format!("year_from={}", yf));
            }
        }
        if let Some(yt) = year_to {
            if !yt.is_empty() {
                params.push(format!("year_to={}", yt));
            }
        }
        if let Some(lang) = language {
            if !lang.is_empty() {
                params.push(format!("language={}", urlencoding::encode(lang)));
            }
        }
        if let Some(dewey) = dewey_code {
            if !dewey.is_empty() {
                params.push(format!("dewey_code={}", urlencoding::encode(dewey)));
            }
        }
        if let Some(has_vols) = has_volumes {
            params.push(format!("has_volumes={}", has_vols));
        }
        if let Some(available) = available_only {
            if available {
                params.push("available=true".to_string());
            }
        }
        if let Some(lid) = location_id {
            if !lid.is_empty() {
                params.push(format!("location_id={}", urlencoding::encode(lid)));
            }
        }

        // Join parameters with &
        url.push_str(&params.join("&"));

        println!("Searching titles with URL: {}", url);

        let response = self.client.get(&url).send()?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_default();
            return Err(format!("API returned status: {} - {}", status, error_text).into());
        }

        // Parse response - backend returns {"results": [...], "total": N, ...}
        let json: serde_json::Value = response.json()?;
        let titles: Vec<TitleWithCount> = serde_json::from_value(
            json.get("results")
                .ok_or("Missing 'results' field in response")?
                .clone()
        )?;

        println!("Search returned {} titles", titles.len());

        Ok(titles)
    }

    /// Creates a new title in the library.
    ///
    /// This method makes a POST request to the `/api/v1/titles` endpoint to create
    /// a new title with the provided metadata. The title is created without any
    /// physical volumes initially (volume count = 0).
    ///
    /// # Arguments
    ///
    /// * `request` - A `CreateTitleRequest` containing all the title metadata:
    ///   - `title`: Title name (required)
    ///   - `subtitle`: Optional subtitle
    ///   - `isbn`: Optional ISBN number
    ///   - `publisher`: Optional publisher name
    ///   - `publication_year`: Optional publication year
    ///   - `pages`: Optional page count
    ///   - `language`: Language code (required)
    ///   - `genre_id`: Optional genre UUID
    ///   - `summary`: Optional book summary/description
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The UUID of the newly created title
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The HTTP request fails
    ///   - The server returns an error (e.g., validation failure, database error)
    ///   - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    /// use rbibli_frontend::models::CreateTitleRequest;
    ///
    /// let client = ApiClient::default();
    /// let request = CreateTitleRequest {
    ///     title: "The Rust Programming Language".to_string(),
    ///     subtitle: Some("Second Edition".to_string()),
    ///     isbn: Some("978-1-59327-828-1".to_string()),
    ///     publisher: Some("No Starch Press".to_string()),
    ///     publication_year: Some(2018),
    ///     pages: Some(552),
    ///     language: "en".to_string(),
    ///     dewey_code: None,
    ///     dewey_category: None,
    ///     genre_id: None,
    ///     summary: Some("The official book on Rust".to_string()),
    ///     cover_url: None,
    /// };
    ///
    /// match client.create_title(request) {
    ///     Ok(id) => println!("Created title with ID: {}", id),
    ///     Err(e) => eprintln!("Failed to create title: {}", e),
    /// }
    /// ```
    pub fn create_title(&self, request: CreateTitleRequest) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/api/v1/titles", self.base_url);

        println!("Creating title: {}", request.title);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("API returned status with error: {}", error_text).into());
        }

        let result: serde_json::Value = response.json()?;
        let title_id = result["id"].as_str()
            .ok_or("No ID in response")?
            .to_string();

        println!("Successfully created title with ID: {}", title_id);

        Ok(title_id)
    }

    /// Updates an existing title's metadata.
    ///
    /// This method makes a PUT request to `/api/v1/titles/{id}` to update one or more
    /// fields of an existing title. Only the fields present in the request (non-None)
    /// will be updated; other fields remain unchanged.
    ///
    /// # Arguments
    ///
    /// * `title_id` - The UUID of the title to update
    /// * `request` - An `UpdateTitleRequest` with the fields to update (all fields are optional)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Title updated successfully
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The title ID is not found (404)
    ///   - The HTTP request fails
    ///   - The server returns an error (e.g., validation failure)
    ///   - No fields were provided for update
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    /// use rbibli_frontend::models::UpdateTitleRequest;
    ///
    /// let client = ApiClient::default();
    /// let request = UpdateTitleRequest {
    ///     title: None,  // Don't change the title
    ///     subtitle: Some("Third Edition".to_string()),  // Update subtitle
    ///     pages: Some(600),  // Update page count
    ///     ..Default::default()
    /// };
    ///
    /// match client.update_title("123e4567-e89b-12d3-a456-426614174000", request) {
    ///     Ok(()) => println!("Title updated successfully"),
    ///     Err(e) => eprintln!("Failed to update title: {}", e),
    /// }
    /// ```
    pub fn update_title(&self, title_id: &str, request: UpdateTitleRequest) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/titles/{}", self.base_url, title_id);

        println!("Updating title: {}", title_id);

        let response = self.client
            .put(&url)
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("API returned status with error: {}", error_text).into());
        }

        println!("Successfully updated title: {}", title_id);

        Ok(())
    }

    /// Deletes a title from the library.
    ///
    /// This method makes a DELETE request to `/api/v1/titles/{id}` to remove a title
    /// from the library database. A title can only be deleted if it has no physical
    /// volumes (copies) associated with it. This business rule prevents accidental
    /// deletion of titles that still have physical inventory.
    ///
    /// # Arguments
    ///
    /// * `title_id` - The UUID of the title to delete
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The title was successfully deleted
    /// * `Err(Box<dyn Error>)` - An error occurred:
    ///   - The HTTP request failed
    ///   - The title was not found (404)
    ///   - The title has volumes and cannot be deleted (409 Conflict)
    ///   - The server returned another error
    ///
    /// # Business Rules
    ///
    /// - A title can only be deleted if it has 0 volumes
    /// - If the title has volumes, a 409 Conflict error is returned
    /// - All volumes must be deleted before the title can be deleted
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    ///
    /// let client = ApiClient::default();
    /// let title_id = "550e8400-e29b-41d4-a716-446655440000";
    ///
    /// match client.delete_title(title_id) {
    ///     Ok(()) => println!("Title deleted successfully"),
    ///     Err(e) => eprintln!("Failed to delete title: {}", e),
    /// }
    /// ```
    pub fn delete_title(&self, title_id: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/titles/{}", self.base_url, title_id);

        println!("Deleting title: {}", title_id);

        let response = self.client
            .delete(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to delete title: {}", error_text).into());
        }

        println!("Successfully deleted title: {}", title_id);

        Ok(())
    }

    /// Fetches all authors for a specific title with their roles.
    ///
    /// # Arguments
    ///
    /// * `title_id` - The UUID of the title to fetch authors for
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<AuthorWithRole>)` - List of authors with their roles and display order
    /// * `Err(Box<dyn Error>)` - If the request fails
    pub fn get_title_authors(&self, title_id: &str) -> Result<Vec<crate::models::AuthorWithRole>, Box<dyn Error>> {
        let url = format!("{}/api/v1/titles/{}/authors", self.base_url, title_id);

        println!("Fetching authors for title: {}", title_id);

        let response = self.client
            .get(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to fetch title authors: {}", error_text).into());
        }

        let authors = response.json::<Vec<crate::models::AuthorWithRole>>()?;
        println!("Successfully fetched {} authors for title", authors.len());

        Ok(authors)
    }

    /// Adds an author to a title with a specified role.
    ///
    /// # Arguments
    ///
    /// * `title_id` - The UUID of the title
    /// * `request` - The request containing author_id, role, and optional display_order
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The ID of the created relationship
    /// * `Err(Box<dyn Error>)` - If the request fails
    pub fn add_author_to_title(&self, title_id: &str, request: crate::models::AddAuthorToTitleRequest) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/api/v1/titles/{}/authors", self.base_url, title_id);

        println!("Adding author {} to title {} with role {:?}", request.author_id, title_id, request.role);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to add author to title: {}", error_text).into());
        }

        let result: serde_json::Value = response.json()?;
        let id = result["id"].as_str()
            .ok_or("Response missing 'id' field")?
            .to_string();

        println!("Successfully added author to title");

        Ok(id)
    }

    /// Removes an author from a title.
    ///
    /// # Arguments
    ///
    /// * `title_id` - The UUID of the title
    /// * `author_id` - The UUID of the author to remove
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the author was successfully removed
    /// * `Err(Box<dyn Error>)` - If the request fails
    pub fn remove_author_from_title(&self, title_id: &str, author_id: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/titles/{}/authors/{}", self.base_url, title_id, author_id);

        println!("Removing author {} from title {}", author_id, title_id);

        let response = self.client
            .delete(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to remove author from title: {}", error_text).into());
        }

        println!("Successfully removed author from title");

        Ok(())
    }

    /// Fetches all storage locations with their full hierarchical paths.
    ///
    /// This method makes a GET request to `/api/v1/locations` to retrieve all physical
    /// storage locations in the library. Each location includes its full hierarchical path
    /// (e.g., "House > Living Room > Bookshelf 1") and level in the hierarchy.
    ///
    /// Locations support parent-child relationships, allowing you to organize storage
    /// in a tree structure (e.g., rooms contain shelves, shelves contain sections).
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<LocationWithPath>)` - A vector of locations with full paths, ordered by path
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The HTTP request fails
    ///   - The server returns an error
    ///   - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    ///
    /// let client = ApiClient::default();
    /// match client.get_locations() {
    ///     Ok(locations) => {
    ///         for loc in locations {
    ///             println!("{} (level {})", loc.full_path, loc.level);
    ///         }
    ///     },
    ///     Err(e) => eprintln!("Failed to fetch locations: {}", e),
    /// }
    /// ```
    pub fn get_locations(&self) -> Result<Vec<LocationWithPath>, Box<dyn Error>> {
        let url = format!("{}/api/v1/locations", self.base_url);

        println!("Fetching locations from: {}", url);

        let response = self.client.get(&url).send()?;

        if !response.status().is_success() {
            return Err(format!("API returned status: {}", response.status()).into());
        }

        let locations: Vec<LocationWithPath> = response.json()?;

        println!("Successfully fetched {} locations", locations.len());

        Ok(locations)
    }

    /// Creates a new storage location.
    ///
    /// This method makes a POST request to `/api/v1/locations` to create a new
    /// physical storage location. The location can optionally be nested under
    /// a parent location to create a hierarchical structure.
    ///
    /// # Arguments
    ///
    /// * `request` - A `CreateLocationRequest` containing:
    ///   - `name`: Location name (required, e.g., "Bookshelf 1")
    ///   - `description`: Optional description
    ///   - `parent_id`: Optional UUID of the parent location for nesting
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The UUID of the newly created location
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The HTTP request fails
    ///   - The parent_id is invalid or doesn't exist
    ///   - The server returns an error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    /// use rbibli_frontend::models::CreateLocationRequest;
    ///
    /// let client = ApiClient::default();
    ///
    /// // Create a root location
    /// let request = CreateLocationRequest {
    ///     name: "Living Room".to_string(),
    ///     description: Some("Main living area".to_string()),
    ///     parent_id: None,
    /// };
    /// let room_id = client.create_location(request).unwrap();
    ///
    /// // Create a nested location
    /// let shelf_request = CreateLocationRequest {
    ///     name: "Bookshelf 1".to_string(),
    ///     description: None,
    ///     parent_id: Some(room_id),
    /// };
    /// client.create_location(shelf_request).unwrap();
    /// ```
    pub fn create_location(&self, request: CreateLocationRequest) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/api/v1/locations", self.base_url);

        println!("Creating location: {}", request.name);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("API returned status with error: {}", error_text).into());
        }

        let result: serde_json::Value = response.json()?;
        let location_id = result["id"].as_str()
            .ok_or("No ID in response")?
            .to_string();

        println!("Successfully created location with ID: {}", location_id);

        Ok(location_id)
    }

    /// Updates an existing storage location.
    ///
    /// This method makes a PUT request to `/api/v1/locations/{id}` to update
    /// a location's information.
    ///
    /// # Arguments
    ///
    /// * `location_id` - The UUID of the location to update
    /// * `request` - The update request containing the fields to change
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Location updated successfully
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The location ID is not found (404)
    ///   - The HTTP request fails
    ///   - The server returns an error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    /// use rbibli_frontend::models::UpdateLocationRequest;
    ///
    /// let client = ApiClient::default();
    /// let request = UpdateLocationRequest {
    ///     name: Some("Shelf A1".to_string()),
    ///     description: Some("Top shelf in room A".to_string()),
    ///     parent_id: None,
    /// };
    ///
    /// match client.update_location("123e4567-e89b-12d3-a456-426614174000", request) {
    ///     Ok(()) => println!("Location updated successfully"),
    ///     Err(e) => eprintln!("Failed to update location: {}", e),
    /// }
    /// ```
    pub fn update_location(&self, location_id: &str, request: UpdateLocationRequest) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/locations/{}", self.base_url, location_id);

        println!("Updating location: {}", location_id);

        let response = self.client
            .put(&url)
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("API returned status with error: {}", error_text).into());
        }

        println!("Successfully updated location: {}", location_id);

        Ok(())
    }

    /// Deletes a storage location by its ID.
    ///
    /// This method makes a DELETE request to `/api/v1/locations/{id}` to permanently
    /// remove a location from the system.
    ///
    /// # Arguments
    ///
    /// * `location_id` - The UUID of the location to delete
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Location deleted successfully
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The location ID is not found (404)
    ///   - The HTTP request fails
    ///   - The location cannot be deleted (e.g., has child locations or volumes stored in it)
    ///   - The server returns an error
    ///
    /// # Warning
    ///
    /// Deleting a location may fail if:
    /// - It has child locations (delete children first)
    /// - It has volumes stored in it (move volumes first)
    /// - Database constraints prevent deletion
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    ///
    /// let client = ApiClient::default();
    /// match client.delete_location("123e4567-e89b-12d3-a456-426614174000") {
    ///     Ok(()) => println!("Location deleted successfully"),
    ///     Err(e) => eprintln!("Failed to delete location: {}", e),
    /// }
    /// ```
    pub fn delete_location(&self, location_id: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/locations/{}", self.base_url, location_id);

        println!("Deleting location: {}", location_id);

        let response = self.client
            .delete(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("API returned status with error: {}", error_text).into());
        }

        println!("Successfully deleted location: {}", location_id);

        Ok(())
    }

    /// Fetches all authors with their title counts from the backend API.
    ///
    /// This method makes a GET request to `/api/v1/authors` to retrieve a list of
    /// all authors in the library database. Each author includes their biographical
    /// information along with a count of how many titles they are associated with.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<AuthorWithTitleCount>)` - A vector of authors sorted by last name, then first name
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The HTTP request fails
    ///   - The server returns an error
    ///   - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    ///
    /// let client = ApiClient::default();
    /// match client.get_authors() {
    ///     Ok(authors) => {
    ///         for author in authors {
    ///             println!("{} {} - {} titles",
    ///                 author.author.first_name,
    ///                 author.author.last_name,
    ///                 author.title_count);
    ///         }
    ///     },
    ///     Err(e) => eprintln!("Failed to fetch authors: {}", e),
    /// }
    /// ```
    pub fn get_authors(&self) -> Result<Vec<AuthorWithTitleCount>, Box<dyn Error>> {
        let url = format!("{}/api/v1/authors", self.base_url);

        println!("Fetching authors from: {}", url);

        let response = self.client.get(&url).send()?;

        if !response.status().is_success() {
            return Err(format!("API returned status: {}", response.status()).into());
        }

        let authors: Vec<AuthorWithTitleCount> = response.json()?;

        println!("Successfully fetched {} authors", authors.len());

        Ok(authors)
    }

    /// Creates a new author in the library database.
    ///
    /// This method makes a POST request to `/api/v1/authors` to add a new author
    /// with biographical information. The author can then be associated with titles.
    ///
    /// # Arguments
    ///
    /// * `request` - A `CreateAuthorRequest` containing:
    ///   - `first_name`: Author's first name (required)
    ///   - `last_name`: Author's last name (required)
    ///   - `biography`: Optional biographical text
    ///   - `birth_date`: Optional birth date in ISO format (YYYY-MM-DD)
    ///   - `death_date`: Optional death date in ISO format (YYYY-MM-DD)
    ///   - `nationality`: Optional nationality
    ///   - `website_url`: Optional website URL
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The UUID of the newly created author
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The HTTP request fails
    ///   - Date format is invalid
    ///   - The server returns an error (e.g., validation failure)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    /// use rbibli_frontend::models::CreateAuthorRequest;
    ///
    /// let client = ApiClient::default();
    /// let request = CreateAuthorRequest {
    ///     first_name: "Isaac".to_string(),
    ///     last_name: "Asimov".to_string(),
    ///     biography: Some("American science fiction writer".to_string()),
    ///     birth_date: Some("1920-01-02".to_string()),
    ///     death_date: Some("1992-04-06".to_string()),
    ///     nationality: Some("American".to_string()),
    ///     website_url: None,
    /// };
    ///
    /// match client.create_author(request) {
    ///     Ok(id) => println!("Created author with ID: {}", id),
    ///     Err(e) => eprintln!("Failed to create author: {}", e),
    /// }
    /// ```
    pub fn create_author(&self, request: CreateAuthorRequest) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/api/v1/authors", self.base_url);

        println!("Creating author: {} {}", request.first_name, request.last_name);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("API returned status with error: {}", error_text).into());
        }

        let result: serde_json::Value = response.json()?;
        let author_id = result["id"].as_str()
            .ok_or("No ID in response")?
            .to_string();

        println!("Successfully created author with ID: {}", author_id);

        Ok(author_id)
    }

    /// Updates an existing author in the library database.
    ///
    /// This method makes a PUT request to `/api/v1/authors/{id}` to update
    /// an author's biographical information.
    ///
    /// # Arguments
    ///
    /// * `author_id` - The UUID of the author to update
    /// * `request` - The update request containing the fields to change
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Author updated successfully
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The author ID is not found (404)
    ///   - The HTTP request fails
    ///   - The server returns an error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    /// use rbibli_frontend::models::UpdateAuthorRequest;
    ///
    /// let client = ApiClient::default();
    /// let request = UpdateAuthorRequest {
    ///     first_name: Some("Isaac".to_string()),
    ///     last_name: Some("Asimov".to_string()),
    ///     biography: Some("American science fiction writer".to_string()),
    ///     birth_date: Some("1920-01-02".to_string()),
    ///     death_date: Some("1992-04-06".to_string()),
    ///     nationality: Some("American".to_string()),
    ///     website_url: None,
    /// };
    ///
    /// match client.update_author("123e4567-e89b-12d3-a456-426614174000", request) {
    ///     Ok(()) => println!("Author updated successfully"),
    ///     Err(e) => eprintln!("Failed to update author: {}", e),
    /// }
    /// ```
    pub fn update_author(&self, author_id: &str, request: UpdateAuthorRequest) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/authors/{}", self.base_url, author_id);

        println!("Updating author: {}", author_id);

        let response = self.client
            .put(&url)
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("API returned status with error: {}", error_text).into());
        }

        println!("Successfully updated author: {}", author_id);

        Ok(())
    }

    /// Deletes an author from the library database.
    ///
    /// This method makes a DELETE request to `/api/v1/authors/{id}` to permanently
    /// remove an author from the system.
    ///
    /// # Arguments
    ///
    /// * `author_id` - The UUID of the author to delete
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Author deleted successfully
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The author ID is not found (404)
    ///   - The HTTP request fails
    ///   - The author cannot be deleted (e.g., has associated titles)
    ///   - The server returns an error
    ///
    /// # Warning
    ///
    /// Deleting an author may fail if they are associated with any titles.
    /// You may need to remove the author-title associations first.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    ///
    /// let client = ApiClient::default();
    /// match client.delete_author("123e4567-e89b-12d3-a456-426614174000") {
    ///     Ok(()) => println!("Author deleted successfully"),
    ///     Err(e) => eprintln!("Failed to delete author: {}", e),
    /// }
    /// ```
    pub fn delete_author(&self, author_id: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/authors/{}", self.base_url, author_id);

        println!("Deleting author: {}", author_id);

        let response = self.client
            .delete(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("API returned status with error: {}", error_text).into());
        }

        println!("Successfully deleted author: {}", author_id);

        Ok(())
    }

    /// Fetches all publishers with their title counts from the backend API.
    ///
    /// This method makes a GET request to `/api/v1/publishers` to retrieve a list of
    /// all publishers in the library database. Each publisher includes their company
    /// information along with a count of how many titles they have published.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<PublisherWithTitleCount>)` - A vector of publishers sorted alphabetically by name
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The HTTP request fails
    ///   - The server returns an error
    ///   - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    ///
    /// let client = ApiClient::default();
    /// match client.get_publishers() {
    ///     Ok(publishers) => {
    ///         for publisher in publishers {
    ///             println!("{} - {} titles",
    ///                 publisher.publisher.name,
    ///                 publisher.title_count);
    ///         }
    ///     },
    ///     Err(e) => eprintln!("Failed to fetch publishers: {}", e),
    /// }
    /// ```
    pub fn get_publishers(&self) -> Result<Vec<PublisherWithTitleCount>, Box<dyn Error>> {
        let url = format!("{}/api/v1/publishers", self.base_url);

        println!("Fetching publishers from: {}", url);

        let response = self.client.get(&url).send()?;

        if !response.status().is_success() {
            return Err(format!("API returned status: {}", response.status()).into());
        }

        let publishers: Vec<PublisherWithTitleCount> = response.json()?;

        println!("Successfully fetched {} publishers", publishers.len());

        Ok(publishers)
    }

    /// Creates a new publisher in the library database.
    ///
    /// This method makes a POST request to `/api/v1/publishers` to add a new publishing
    /// company to the system. The publisher can then be associated with titles.
    ///
    /// # Arguments
    ///
    /// * `request` - A `CreatePublisherRequest` containing:
    ///   - `name`: Publisher name (required)
    ///   - `description`: Optional description of the publisher
    ///   - `website_url`: Optional website URL
    ///   - `country`: Optional country of origin
    ///   - `founded_year`: Optional year the publisher was founded
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The UUID of the newly created publisher
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The HTTP request fails
    ///   - The server returns an error (e.g., validation failure, duplicate name)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    /// use rbibli_frontend::models::CreatePublisherRequest;
    ///
    /// let client = ApiClient::default();
    /// let request = CreatePublisherRequest {
    ///     name: "O'Reilly Media".to_string(),
    ///     description: Some("Technical and computer book publisher".to_string()),
    ///     website_url: Some("https://www.oreilly.com".to_string()),
    ///     country: Some("United States".to_string()),
    ///     founded_year: Some(1978),
    /// };
    ///
    /// match client.create_publisher(request) {
    ///     Ok(id) => println!("Created publisher with ID: {}", id),
    ///     Err(e) => eprintln!("Failed to create publisher: {}", e),
    /// }
    /// ```
    pub fn create_publisher(&self, request: CreatePublisherRequest) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/api/v1/publishers", self.base_url);

        println!("Creating publisher: {}", request.name);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("API returned status with error: {}", error_text).into());
        }

        let result: serde_json::Value = response.json()?;
        let publisher_id = result["id"].as_str()
            .ok_or("No ID in response")?
            .to_string();

        println!("Successfully created publisher with ID: {}", publisher_id);

        Ok(publisher_id)
    }

    /// Updates an existing publisher's information.
    ///
    /// This method makes a PUT request to `/api/v1/publishers/{id}` to update one or
    /// more fields of an existing publisher. Only the fields present in the request
    /// (non-None) will be updated; other fields remain unchanged.
    ///
    /// # Arguments
    ///
    /// * `publisher_id` - The UUID of the publisher to update
    /// * `request` - An `UpdatePublisherRequest` with the fields to update (all optional)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Publisher updated successfully
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The publisher ID is not found (404)
    ///   - The HTTP request fails
    ///   - The server returns an error (e.g., validation failure)
    ///   - No fields were provided for update
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    /// use rbibli_frontend::models::UpdatePublisherRequest;
    ///
    /// let client = ApiClient::default();
    /// let request = UpdatePublisherRequest {
    ///     name: None,  // Don't change the name
    ///     website_url: Some("https://new-website.com".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// match client.update_publisher("123e4567-e89b-12d3-a456-426614174000", request) {
    ///     Ok(()) => println!("Publisher updated successfully"),
    ///     Err(e) => eprintln!("Failed to update publisher: {}", e),
    /// }
    /// ```
    pub fn update_publisher(&self, publisher_id: &str, request: UpdatePublisherRequest) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/publishers/{}", self.base_url, publisher_id);

        println!("Updating publisher: {}", publisher_id);

        let response = self.client
            .put(&url)
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("API returned status with error: {}", error_text).into());
        }

        println!("Successfully updated publisher: {}", publisher_id);

        Ok(())
    }

    /// Deletes a publisher from the library database.
    ///
    /// This method makes a DELETE request to `/api/v1/publishers/{id}` to permanently
    /// remove a publisher from the system.
    ///
    /// # Arguments
    ///
    /// * `publisher_id` - The UUID of the publisher to delete
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Publisher deleted successfully
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The publisher ID is not found (404)
    ///   - The HTTP request fails
    ///   - The publisher cannot be deleted (e.g., has associated titles)
    ///   - The server returns an error
    ///
    /// # Warning
    ///
    /// Deleting a publisher may fail if there are titles associated with it.
    /// You may need to update those titles to use a different publisher first.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    ///
    /// let client = ApiClient::default();
    /// match client.delete_publisher("123e4567-e89b-12d3-a456-426614174000") {
    ///     Ok(()) => println!("Publisher deleted successfully"),
    ///     Err(e) => eprintln!("Failed to delete publisher: {}", e),
    /// }
    /// ```
    pub fn delete_publisher(&self, publisher_id: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/publishers/{}", self.base_url, publisher_id);

        println!("Deleting publisher: {}", publisher_id);

        let response = self.client
            .delete(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("API returned status with error: {}", error_text).into());
        }

        println!("Successfully deleted publisher: {}", publisher_id);

        Ok(())
    }

    /// Fetches all genres with their title counts from the backend API.
    ///
    /// This method makes a GET request to `/api/v1/genres` to retrieve a list of all
    /// book genres/categories in the library. Each genre includes its information along
    /// with a count of how many titles are categorized under that genre.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<GenreWithTitleCount>)` - A vector of genres sorted alphabetically by name
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The HTTP request fails
    ///   - The server returns an error
    ///   - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    ///
    /// let client = ApiClient::default();
    /// match client.get_genres() {
    ///     Ok(genres) => {
    ///         for genre in genres {
    ///             println!("{} - {} titles",
    ///                 genre.genre.name,
    ///                 genre.title_count);
    ///         }
    ///     },
    ///     Err(e) => eprintln!("Failed to fetch genres: {}", e),
    /// }
    /// ```
    pub fn get_genres(&self) -> Result<Vec<GenreWithTitleCount>, Box<dyn Error>> {
        let url = format!("{}/api/v1/genres", self.base_url);

        println!("Fetching genres from: {}", url);

        let response = self.client.get(&url).send()?;

        if !response.status().is_success() {
            return Err(format!("API returned status: {}", response.status()).into());
        }

        let genres: Vec<GenreWithTitleCount> = response.json()?;

        println!("Successfully fetched {} genres", genres.len());

        Ok(genres)
    }

    /// Creates a new genre/category in the library database.
    ///
    /// This method makes a POST request to `/api/v1/genres` to add a new book
    /// genre or category. Genres help organize titles by type (e.g., Fiction,
    /// Science Fiction, Biography, etc.).
    ///
    /// # Arguments
    ///
    /// * `request` - A `CreateGenreRequest` containing:
    ///   - `name`: Genre name (required, e.g., "Science Fiction")
    ///   - `description`: Optional description of the genre
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The UUID of the newly created genre
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The HTTP request fails
    ///   - The server returns an error (e.g., duplicate genre name)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    /// use rbibli_frontend::models::CreateGenreRequest;
    ///
    /// let client = ApiClient::default();
    /// let request = CreateGenreRequest {
    ///     name: "Science Fiction".to_string(),
    ///     description: Some("Speculative fiction based on scientific concepts".to_string()),
    /// };
    ///
    /// match client.create_genre(request) {
    ///     Ok(id) => println!("Created genre with ID: {}", id),
    ///     Err(e) => eprintln!("Failed to create genre: {}", e),
    /// }
    /// ```
    pub fn create_genre(&self, request: CreateGenreRequest) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/api/v1/genres", self.base_url);

        println!("Creating genre: {}", request.name);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("API returned status with error: {}", error_text).into());
        }

        let result: serde_json::Value = response.json()?;
        let genre_id = result["id"].as_str()
            .ok_or("No ID in response")?
            .to_string();

        println!("Successfully created genre with ID: {}", genre_id);

        Ok(genre_id)
    }

    /// Updates an existing genre's information.
    ///
    /// This method makes a PUT request to `/api/v1/genres/{id}` to update one or
    /// more fields of an existing genre. Only the fields present in the request
    /// (non-None) will be updated; other fields remain unchanged.
    ///
    /// # Arguments
    ///
    /// * `genre_id` - The UUID of the genre to update
    /// * `request` - An `UpdateGenreRequest` with the fields to update (all optional)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Genre updated successfully
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The genre ID is not found (404)
    ///   - The HTTP request fails
    ///   - The server returns an error (e.g., validation failure, duplicate name)
    ///   - No fields were provided for update
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    /// use rbibli_frontend::models::UpdateGenreRequest;
    ///
    /// let client = ApiClient::default();
    /// let request = UpdateGenreRequest {
    ///     name: Some("Sci-Fi".to_string()),
    ///     description: Some("Science Fiction and Fantasy".to_string()),
    /// };
    ///
    /// match client.update_genre("123e4567-e89b-12d3-a456-426614174000", request) {
    ///     Ok(()) => println!("Genre updated successfully"),
    ///     Err(e) => eprintln!("Failed to update genre: {}", e),
    /// }
    /// ```
    pub fn update_genre(&self, genre_id: &str, request: UpdateGenreRequest) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/genres/{}", self.base_url, genre_id);

        println!("Updating genre: {}", genre_id);

        let response = self.client
            .put(&url)
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("API returned status with error: {}", error_text).into());
        }

        println!("Successfully updated genre: {}", genre_id);

        Ok(())
    }

    /// Deletes a genre from the library database.
    ///
    /// This method makes a DELETE request to `/api/v1/genres/{id}` to permanently
    /// remove a genre/category from the system.
    ///
    /// # Arguments
    ///
    /// * `genre_id` - The UUID of the genre to delete
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Genre deleted successfully
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The genre ID is not found (404)
    ///   - The HTTP request fails
    ///   - The genre cannot be deleted (e.g., has titles categorized under it)
    ///   - The server returns an error
    ///
    /// # Warning
    ///
    /// Deleting a genre may fail if there are titles categorized under it.
    /// You may need to recategorize those titles to a different genre first.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rbibli_frontend::api_client::ApiClient;
    ///
    /// let client = ApiClient::default();
    /// match client.delete_genre("123e4567-e89b-12d3-a456-426614174000") {
    ///     Ok(()) => println!("Genre deleted successfully"),
    ///     Err(e) => eprintln!("Failed to delete genre: {}", e),
    /// }
    /// ```
    pub fn delete_genre(&self, genre_id: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/genres/{}", self.base_url, genre_id);

        println!("Deleting genre: {}", genre_id);

        let response = self.client
            .delete(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("API returned status with error: {}", error_text).into());
        }

        println!("Successfully deleted genre: {}", genre_id);

        Ok(())
    }

    // ========================================================================
    // Series Operations
    // ========================================================================

    /// Fetches all series with their title counts from the backend API.
    pub fn get_series(&self) -> Result<Vec<SeriesWithTitleCount>, Box<dyn Error>> {
        let url = format!("{}/api/v1/series", self.base_url);

        println!("Fetching series from: {}", url);

        let response = self.client.get(&url).send()?;

        if !response.status().is_success() {
            return Err(format!("API returned status: {}", response.status()).into());
        }

        let series: Vec<SeriesWithTitleCount> = response.json()?;

        println!("Successfully fetched {} series", series.len());

        Ok(series)
    }

    /// Creates a new series in the library database.
    pub fn create_series(&self, request: CreateSeriesRequest) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/api/v1/series", self.base_url);

        println!("Creating series: {}", request.name);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("API returned status with error: {}", error_text).into());
        }

        let result: serde_json::Value = response.json()?;
        let series_id = result["id"].as_str()
            .ok_or("No ID in response")?
            .to_string();

        println!("Successfully created series with ID: {}", series_id);

        Ok(series_id)
    }

    /// Updates an existing series's information.
    pub fn update_series(&self, series_id: &str, request: UpdateSeriesRequest) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/series/{}", self.base_url, series_id);

        println!("Updating series: {}", series_id);

        let response = self.client
            .put(&url)
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("API returned status with error: {}", error_text).into());
        }

        println!("Successfully updated series: {}", series_id);

        Ok(())
    }

    /// Deletes a series from the library database.
    pub fn delete_series(&self, series_id: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/series/{}", self.base_url, series_id);

        println!("Deleting series: {}", series_id);

        let response = self.client
            .delete(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("API returned status with error: {}", error_text).into());
        }

        println!("Successfully deleted series: {}", series_id);

        Ok(())
    }

    // ========================================================================
    // Volume Operations
    // ========================================================================

    /// Fetches all volumes for a specific title from the backend API.
    ///
    /// This method makes a GET request to `/api/v1/titles/{title_id}/volumes` to retrieve
    /// all physical volumes (copies) associated with a specific title.
    ///
    /// # Arguments
    ///
    /// * `title_id` - The UUID of the title to fetch volumes for
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Volume>)` - A vector of volumes for the title on success
    /// * `Err(Box<dyn Error>)` - An error if the request fails
    pub fn get_volumes_for_title(&self, title_id: &str) -> Result<Vec<Volume>, Box<dyn Error>> {
        let url = format!("{}/api/v1/titles/{}/volumes", self.base_url, title_id);

        println!("Fetching volumes for title: {}", title_id);

        let response = self.client.get(&url).send()?;

        if !response.status().is_success() {
            return Err(format!("API returned status: {}", response.status()).into());
        }

        let volumes: Vec<Volume> = response.json()?;

        println!("Successfully fetched {} volumes", volumes.len());

        Ok(volumes)
    }

    /// Creates a new volume for a title.
    ///
    /// This method makes a POST request to `/api/v1/volumes` to create a new physical
    /// volume. The copy_number is automatically calculated by the backend.
    ///
    /// # Arguments
    ///
    /// * `request` - A `CreateVolumeRequest` containing:
    ///   - `title_id`: The title this volume belongs to
    ///   - `barcode`: Unique barcode (VOL-XXXXXX format)
    ///   - `condition`: Physical condition of the volume
    ///   - `location_id`: Optional location where volume is stored
    ///   - `individual_notes`: Optional notes about this specific volume
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Volume created successfully
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The barcode is invalid or already exists (409 Conflict)
    ///   - The HTTP request fails
    ///   - The title ID is not found
    pub fn create_volume(&self, request: CreateVolumeRequest) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/volumes", self.base_url);

        println!("Creating volume with barcode: {}", request.barcode);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to create volume: {}", error_text).into());
        }

        println!("Successfully created volume");

        Ok(())
    }

    /// Updates an existing volume's information.
    ///
    /// This method makes a PUT request to `/api/v1/volumes/{id}` to update
    /// a volume's details. Only provided fields will be updated (partial update).
    ///
    /// # Arguments
    ///
    /// * `volume_id` - The UUID of the volume to update
    /// * `request` - An `UpdateVolumeRequest` with fields to update
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Volume updated successfully
    /// * `Err(Box<dyn Error>)` - An error if the request fails or volume not found
    pub fn update_volume(&self, volume_id: &str, request: UpdateVolumeRequest) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/volumes/{}", self.base_url, volume_id);

        println!("Updating volume: {}", volume_id);

        let response = self.client
            .put(&url)
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to update volume: {}", error_text).into());
        }

        println!("Successfully updated volume: {}", volume_id);

        Ok(())
    }

    /// Deletes a volume from the library.
    ///
    /// This method makes a DELETE request to `/api/v1/volumes/{id}` to permanently
    /// remove a physical volume from the system.
    ///
    /// # Arguments
    ///
    /// * `volume_id` - The UUID of the volume to delete
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Volume deleted successfully
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - The volume is currently loaned or overdue (409 Conflict)
    ///   - The volume ID is not found (404)
    ///   - The HTTP request fails
    ///
    /// # Business Rules
    ///
    /// A volume cannot be deleted if it is currently loaned or overdue.
    /// The backend will return a 409 Conflict error in this case.
    pub fn delete_volume(&self, volume_id: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/volumes/{}", self.base_url, volume_id);

        println!("Deleting volume: {}", volume_id);

        let response = self.client
            .delete(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to delete volume: {}", error_text).into());
        }

        println!("Successfully deleted volume: {}", volume_id);

        Ok(())
    }
}

impl Default for ApiClient {
    /// Creates a default API client configured for local development.
    ///
    /// This implementation provides a convenient way to create an `ApiClient` instance
    /// that connects to a backend server running locally on `http://localhost:8000`.
    /// This is the typical configuration for development and testing.
    ///
    /// # Returns
    ///
    /// An `ApiClient` instance configured to connect to `http://localhost:8000`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbibli_frontend::api_client::ApiClient;
    ///
    /// // Create a client using the default configuration
    /// let client = ApiClient::default();
    ///
    /// // Equivalent to:
    /// let explicit_client = ApiClient::new("http://localhost:8000".to_string());
    /// ```
    ///
    /// # Usage in Production
    ///
    /// For production deployments, use `ApiClient::new()` with the appropriate server URL
    /// instead of relying on this default implementation.
    fn default() -> Self {
        // Default to localhost:8000
        Self::new("http://localhost:8000".to_string())
    }
}

impl ApiClient {
    /// Uploads a cover image for a title.
    ///
    /// This method uploads an image file to the backend API and associates it with a specific title.
    /// The image is stored as a BLOB in the database. The method uses multipart/form-data encoding
    /// to send both the title ID and the image file data.
    ///
    /// # Arguments
    ///
    /// * `title_id` - The UUID string of the title to attach the image to
    /// * `image_data` - The raw bytes of the image file
    /// * `filename` - The original filename of the image (used for MIME type detection)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Image was uploaded successfully
    /// * `Err(Box<dyn Error>)` - Upload failed (network error, invalid data, title not found, etc.)
    ///
    /// # Examples
    ///
    /// ```
    /// use rbibli_frontend::api_client::ApiClient;
    /// use std::fs;
    ///
    /// let client = ApiClient::default();
    /// let image_data = fs::read("cover.jpg").unwrap();
    /// client.upload_cover_image("title-uuid".to_string(), image_data, "cover.jpg".to_string())?;
    /// ```
    pub fn upload_cover_image(
        &self,
        title_id: String,
        image_data: Vec<u8>,
        filename: String,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/uploads/cover", self.base_url);

        // Create multipart form
        let part = reqwest::blocking::multipart::Part::bytes(image_data)
            .file_name(filename);

        let form = reqwest::blocking::multipart::Form::new()
            .text("title_id", title_id)
            .part("cover", part);

        // Send the request
        let response = self.client.post(&url).multipart(form).send()?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Failed to upload image: {}", error_text).into())
        }
    }

    /// Looks up book information by ISBN from Google Books API.
    ///
    /// This method fetches comprehensive book metadata including title, authors, publisher,
    /// description, and cover image from Google Books. The cover image is downloaded and
    /// returned as base64-encoded data ready to be stored in the database.
    ///
    /// # Arguments
    ///
    /// * `isbn` - The ISBN-10 or ISBN-13 number (with or without hyphens)
    ///
    /// # Returns
    ///
    /// * `Ok(IsbnLookupResponse)` - Book data including base64-encoded cover image
    /// * `Err(Box<dyn Error>)` - If ISBN is not found or lookup fails
    ///
    /// # Examples
    ///
    /// ```
    /// use rbibli_frontend::api_client::ApiClient;
    ///
    /// let client = ApiClient::default();
    /// let book = client.lookup_isbn("9780134685991".to_string())?;
    /// println!("Found: {}", book.title);
    /// ```
    pub fn lookup_isbn(&self, isbn: String) -> Result<IsbnLookupResponse, Box<dyn Error>> {
        let url = format!("{}/api/v1/isbn/{}", self.base_url, isbn);

        let response = self.client.get(&url).send()?;

        if response.status().is_success() {
            let book_data: IsbnLookupResponse = response.json()?;
            Ok(book_data)
        } else {
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("ISBN lookup failed: {}", error_text).into())
        }
    }

    // ========== Borrower Groups API ==========

    /// Fetches all borrower groups from the backend API.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<BorrowerGroup>)` - A vector of borrower groups on success
    /// * `Err(Box<dyn Error>)` - An error if the request fails
    pub fn get_borrower_groups(&self) -> Result<Vec<BorrowerGroup>, Box<dyn Error>> {
        let url = format!("{}/api/v1/borrower-groups", self.base_url);

        println!("Fetching borrower groups from: {}", url);

        let response = self.client.get(&url).send()?;

        if !response.status().is_success() {
            return Err(format!("API returned status: {}", response.status()).into());
        }

        let groups: Vec<BorrowerGroup> = response.json()?;
        println!("Successfully fetched {} borrower groups", groups.len());

        Ok(groups)
    }

    /// Creates a new borrower group.
    ///
    /// # Arguments
    ///
    /// * `request` - The borrower group data including name, loan duration, and description
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The ID of the created group
    /// * `Err(Box<dyn Error>)` - An error if creation fails
    pub fn create_borrower_group(&self, request: &CreateBorrowerGroupRequest) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/api/v1/borrower-groups", self.base_url);

        println!("Creating borrower group: {}", request.name);

        let response = self.client
            .post(&url)
            .json(request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to create borrower group: {}", error_text).into());
        }

        let result: serde_json::Value = response.json()?;
        let id = result["id"].as_str().unwrap_or("").to_string();

        println!("Successfully created borrower group: {}", id);
        Ok(id)
    }

    /// Updates an existing borrower group.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The ID of the group to update
    /// * `request` - The updated group data
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Update was successful
    /// * `Err(Box<dyn Error>)` - An error if update fails
    pub fn update_borrower_group(&self, group_id: &str, request: &UpdateBorrowerGroupRequest) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/borrower-groups/{}", self.base_url, group_id);

        println!("Updating borrower group: {}", group_id);

        let response = self.client
            .put(&url)
            .json(request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to update borrower group: {}", error_text).into());
        }

        println!("Successfully updated borrower group: {}", group_id);
        Ok(())
    }

    /// Deletes a borrower group.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The ID of the group to delete
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Deletion was successful
    /// * `Err(Box<dyn Error>)` - An error if deletion fails
    pub fn delete_borrower_group(&self, group_id: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/borrower-groups/{}", self.base_url, group_id);

        println!("Deleting borrower group: {}", group_id);

        let response = self.client
            .delete(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to delete borrower group: {}", error_text).into());
        }

        println!("Successfully deleted borrower group: {}", group_id);
        Ok(())
    }

    // ========== Borrowers API ==========

    /// Fetches all borrowers with their group information from the backend API.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<BorrowerWithGroup>)` - A vector of borrowers with group info on success
    /// * `Err(Box<dyn Error>)` - An error if the request fails
    pub fn get_borrowers(&self) -> Result<Vec<BorrowerWithGroup>, Box<dyn Error>> {
        let url = format!("{}/api/v1/borrowers", self.base_url);

        println!("Fetching borrowers from: {}", url);

        let response = self.client.get(&url).send()?;

        if !response.status().is_success() {
            return Err(format!("API returned status: {}", response.status()).into());
        }

        let borrowers: Vec<BorrowerWithGroup> = response.json()?;
        println!("Successfully fetched {} borrowers", borrowers.len());

        Ok(borrowers)
    }

    /// Creates a new borrower.
    ///
    /// # Arguments
    ///
    /// * `request` - The borrower data including name, email, phone, and group
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The ID of the created borrower
    /// * `Err(Box<dyn Error>)` - An error if creation fails
    pub fn create_borrower(&self, request: &CreateBorrowerRequest) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/api/v1/borrowers", self.base_url);

        println!("Creating borrower: {}", request.name);

        let response = self.client
            .post(&url)
            .json(request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to create borrower: {}", error_text).into());
        }

        let result: serde_json::Value = response.json()?;
        let id = result["id"].as_str().unwrap_or("").to_string();

        println!("Successfully created borrower: {}", id);
        Ok(id)
    }

    /// Updates an existing borrower.
    ///
    /// # Arguments
    ///
    /// * `borrower_id` - The ID of the borrower to update
    /// * `request` - The updated borrower data
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Update was successful
    /// * `Err(Box<dyn Error>)` - An error if update fails
    pub fn update_borrower(&self, borrower_id: &str, request: &UpdateBorrowerRequest) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/borrowers/{}", self.base_url, borrower_id);

        println!("Updating borrower: {}", borrower_id);

        let response = self.client
            .put(&url)
            .json(request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to update borrower: {}", error_text).into());
        }

        println!("Successfully updated borrower: {}", borrower_id);
        Ok(())
    }

    /// Deletes a borrower.
    ///
    /// # Arguments
    ///
    /// * `borrower_id` - The ID of the borrower to delete
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Deletion was successful
    /// * `Err(Box<dyn Error>)` - An error if deletion fails
    pub fn delete_borrower(&self, borrower_id: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/borrowers/{}", self.base_url, borrower_id);

        println!("Deleting borrower: {}", borrower_id);

        let response = self.client
            .delete(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to delete borrower: {}", error_text).into());
        }

        println!("Successfully deleted borrower: {}", borrower_id);
        Ok(())
    }

    // ========== Loans API ==========

    /// Fetches all active loans from the backend API.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<LoanDetail>)` - A vector of active loans with full details
    /// * `Err(Box<dyn Error>)` - An error if the request fails
    pub fn get_active_loans(&self) -> Result<Vec<LoanDetail>, Box<dyn Error>> {
        let url = format!("{}/api/v1/loans", self.base_url);

        println!("Fetching active loans from: {}", url);

        let response = self.client.get(&url).send()?;

        if !response.status().is_success() {
            return Err(format!("API returned status: {}", response.status()).into());
        }

        let loans: Vec<LoanDetail> = response.json()?;
        println!("Successfully fetched {} active loans", loans.len());

        Ok(loans)
    }

    /// Fetches all overdue loans from the backend API.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<LoanDetail>)` - A vector of overdue loans with full details
    /// * `Err(Box<dyn Error>)` - An error if the request fails
    pub fn get_overdue_loans(&self) -> Result<Vec<LoanDetail>, Box<dyn Error>> {
        let url = format!("{}/api/v1/loans/overdue", self.base_url);

        println!("Fetching overdue loans from: {}", url);

        let response = self.client.get(&url).send()?;

        if !response.status().is_success() {
            return Err(format!("API returned status: {}", response.status()).into());
        }

        let loans: Vec<LoanDetail> = response.json()?;
        println!("Successfully fetched {} overdue loans", loans.len());

        Ok(loans)
    }

    /// Creates a new loan by scanning a barcode.
    ///
    /// This method performs the full loan workflow:
    /// 1. Finds the volume by barcode
    /// 2. Validates the volume is loanable and available
    /// 3. Gets the borrower's group loan duration
    /// 4. Creates the loan record
    /// 5. Updates the volume status to loaned
    ///
    /// # Arguments
    ///
    /// * `request` - Contains borrower_id and barcode
    ///
    /// # Returns
    ///
    /// * `Ok(CreateLoanResponse)` - The created loan details including due date
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - Volume is not found
    ///   - Volume is not loanable (damaged)
    ///   - Volume is already loaned
    ///   - Borrower is not found
    pub fn create_loan_by_barcode(&self, request: &CreateLoanRequest) -> Result<CreateLoanResponse, Box<dyn Error>> {
        let url = format!("{}/api/v1/loans", self.base_url);

        println!("Creating loan for borrower: {}, barcode: {}", request.borrower_id, request.barcode);

        let response = self.client
            .post(&url)
            .json(request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to create loan: {}", error_text).into());
        }

        let result: CreateLoanResponse = response.json()?;
        println!("Successfully created loan: {}", result.id);

        Ok(result)
    }

    /// Returns a loaned volume.
    ///
    /// This method processes a return by:
    /// 1. Updating the loan record with return date and status
    /// 2. Updating the volume status to available
    ///
    /// # Arguments
    ///
    /// * `loan_id` - The ID of the loan to return
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Return was processed successfully
    /// * `Err(Box<dyn Error>)` - An error if:
    ///   - Loan is not found
    ///   - Loan is already returned
    pub fn return_loan(&self, loan_id: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/loans/{}/return", self.base_url, loan_id);

        println!("Returning loan: {}", loan_id);

        let response = self.client
            .post(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to return loan: {}", error_text).into());
        }

        println!("Successfully returned loan: {}", loan_id);
        Ok(())
    }

    /// Extend a loan by adding the same duration as the original loan period
    pub fn extend_loan(&self, loan_id: &str) -> Result<LoanDetail, Box<dyn Error>> {
        let url = format!("{}/api/v1/loans/{}/extend", self.base_url, loan_id);

        println!("Extending loan: {}", loan_id);

        let response = self.client
            .post(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to extend loan: {}", error_text).into());
        }

        println!("Successfully extended loan: {}", loan_id);

        // After extending, fetch the updated loan details
        self.get_active_loans()?
            .into_iter()
            .find(|loan| loan.loan.id == loan_id)
            .ok_or_else(|| "Extended loan not found in active loans".into())
    }

    // ========================================================================
    // Dewey Classification API Methods
    // ========================================================================

    /// Search Dewey classifications by keyword
    ///
    /// # Arguments
    ///
    /// * `query` - Search query string (e.g., "mathematics", "poetry")
    /// * `limit` - Maximum number of results to return (default: 20)
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<DeweySearchResult>)` - List of matching classifications
    /// * `Err` - If the request fails
    pub fn search_dewey(&self, query: &str, limit: Option<i32>) -> Result<Vec<DeweySearchResult>, Box<dyn Error>> {
        let limit = limit.unwrap_or(20);
        let url = format!("{}/api/v1/dewey/search?q={}&limit={}",
            self.base_url,
            urlencoding::encode(query),
            limit
        );

        println!("Searching Dewey: query='{}', limit={}", query, limit);

        let response = self.client
            .get(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to search Dewey: {}", error_text).into());
        }

        let results: Vec<DeweySearchResult> = response.json()?;
        println!("Found {} Dewey classifications", results.len());
        Ok(results)
    }

    /// Get Dewey classification by code
    ///
    /// # Arguments
    ///
    /// * `code` - Dewey code (e.g., "515", "813.5")
    ///
    /// # Returns
    ///
    /// * `Ok(DeweySearchResult)` - The classification details
    /// * `Err` - If the request fails or classification not found
    pub fn get_dewey_by_code(&self, code: &str) -> Result<DeweySearchResult, Box<dyn Error>> {
        let url = format!("{}/api/v1/dewey/{}", self.base_url, code);

        println!("Getting Dewey classification: {}", code);

        let response = self.client
            .get(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to get Dewey classification: {}", error_text).into());
        }

        let classification: DeweySearchResult = response.json()?;
        println!("Found classification: {} - {}", classification.code, classification.name);
        Ok(classification)
    }

    // ========================================================================
    // Statistics API methods
    // ========================================================================

    /// Get overall library statistics
    ///
    /// # Returns
    ///
    /// * `Ok(LibraryStatistics)` - Library-wide counts
    /// * `Err` - If the request fails
    pub fn get_library_statistics(&self) -> Result<LibraryStatistics, Box<dyn Error>> {
        let url = format!("{}/api/v1/statistics/library", self.base_url);

        println!("Fetching library statistics...");

        let response = self.client
            .get(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to fetch library statistics: {}", error_text).into());
        }

        let stats: LibraryStatistics = response.json()?;
        println!("Library statistics fetched successfully");
        Ok(stats)
    }

    /// Get volumes per genre statistics
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<GenreStatistic>)` - List of genre statistics ordered by volume count
    /// * `Err` - If the request fails
    pub fn get_genre_statistics(&self) -> Result<Vec<GenreStatistic>, Box<dyn Error>> {
        let url = format!("{}/api/v1/statistics/genres", self.base_url);

        println!("Fetching genre statistics...");

        let response = self.client
            .get(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to fetch genre statistics: {}", error_text).into());
        }

        let stats: Vec<GenreStatistic> = response.json()?;
        println!("Found statistics for {} genres", stats.len());
        Ok(stats)
    }

    /// Get volumes per location statistics
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<LocationStatistic>)` - List of location statistics ordered by volume count
    /// * `Err` - If the request fails
    pub fn get_location_statistics(&self) -> Result<Vec<LocationStatistic>, Box<dyn Error>> {
        let url = format!("{}/api/v1/statistics/locations", self.base_url);

        println!("Fetching location statistics...");

        let response = self.client
            .get(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to fetch location statistics: {}", error_text).into());
        }

        let stats: Vec<LocationStatistic> = response.json()?;
        println!("Found statistics for {} locations", stats.len());
        Ok(stats)
    }

    /// Get loan status statistics
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<LoanStatistic>)` - List of loan status counts
    /// * `Err` - If the request fails
    pub fn get_loan_statistics(&self) -> Result<Vec<LoanStatistic>, Box<dyn Error>> {
        let url = format!("{}/api/v1/statistics/loans", self.base_url);

        println!("Fetching loan statistics...");

        let response = self.client
            .get(&url)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            return Err(format!("Failed to fetch loan statistics: {}", error_text).into());
        }

        let stats: Vec<LoanStatistic> = response.json()?;
        println!("Found {} loan status types", stats.len());
        Ok(stats)
    }
}
