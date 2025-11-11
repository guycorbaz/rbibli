use crate::models::TitleWithCount;
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
}

impl Default for ApiClient {
    fn default() -> Self {
        // Default to localhost:8000
        Self::new("http://localhost:8000".to_string())
    }
}
