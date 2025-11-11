use crate::models::{TitleWithCount, CreateTitleRequest, UpdateTitleRequest, LocationWithPath, CreateLocationRequest, AuthorWithTitleCount, CreateAuthorRequest, PublisherWithTitleCount, CreatePublisherRequest, UpdatePublisherRequest};
use std::error::Error;

/// API client for communicating with the rbibli backend
pub struct ApiClient {
    base_url: String,
    client: reqwest::blocking::Client,
}

impl ApiClient {
    /// Create a new API client with the specified base URL
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::blocking::Client::new(),
        }
    }

    /// Fetch all titles from the backend
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

    /// Create a new title
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

    /// Update a title by ID
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

    /// Fetch all locations from the backend with hierarchical paths
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

    /// Create a new location
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

    /// Delete a location by ID
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

    /// Fetch all authors from the backend with title counts
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

    /// Create a new author
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

    /// Delete an author by ID
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

    /// Fetch all publishers from the backend with title counts
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

    /// Create a new publisher
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

    /// Update a publisher by ID
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

    /// Delete a publisher by ID
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
}

impl Default for ApiClient {
    fn default() -> Self {
        // Default to localhost:8000
        Self::new("http://localhost:8000".to_string())
    }
}
