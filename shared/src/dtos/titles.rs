use serde::{Deserialize, Serialize};

/// Request payload for creating a new title.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTitleRequest {
    pub title: String,
    pub subtitle: Option<String>,
    pub isbn: Option<String>,
    pub publisher: Option<String>,
    pub publisher_id: Option<String>,
    pub publication_year: Option<i32>,
    pub pages: Option<i32>,
    pub language: String,
    pub dewey_code: Option<String>,
    #[serde(alias = "genre")]
    pub genre_id: Option<String>,
    pub series_id: Option<String>,
    pub series_number: Option<String>,
    pub summary: Option<String>,
    pub cover_url: Option<String>,
}

/// Request payload for updating an existing title.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTitleRequest {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub isbn: Option<String>,
    pub publisher: Option<String>,
    pub publisher_id: Option<String>,
    pub publication_year: Option<i32>,
    pub pages: Option<i32>,
    pub language: Option<String>,
    pub dewey_code: Option<String>,
    #[serde(alias = "genre")]
    pub genre_id: Option<String>,
    pub series_id: Option<String>,
    pub series_number: Option<String>,
    pub summary: Option<String>,
    pub cover_url: Option<String>,
}

/// Parameters for advanced title search and filtering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleSearchParams {
    pub q: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub isbn: Option<String>,
    pub series_id: Option<String>,
    pub author_id: Option<String>,
    pub genre_id: Option<String>,
    pub publisher_id: Option<String>,
    pub year_from: Option<i32>,
    pub year_to: Option<i32>,
    pub language: Option<String>,
    pub dewey_code: Option<String>,
    pub has_volumes: Option<bool>,
    pub available: Option<bool>,
    pub location_id: Option<String>,
    #[serde(default = "default_sort_by")]
    pub sort_by: String,
    #[serde(default = "default_sort_order")]
    pub sort_order: String,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_sort_by() -> String {
    "title".to_string()
}

fn default_sort_order() -> String {
    "asc".to_string()
}

fn default_limit() -> i64 {
    100
}

impl TitleSearchParams {
    pub fn validate(&mut self) -> Result<(), String> {
        match self.sort_by.as_str() {
            "title" | "publication_year" | "created_at" => {},
            _ => return Err(format!("Invalid sort_by field: {}. Must be one of: title, publication_year, created_at", self.sort_by)),
        }

        match self.sort_order.as_str() {
            "asc" | "desc" => {},
            _ => return Err(format!("Invalid sort_order: {}. Must be asc or desc", self.sort_order)),
        }

        if self.limit < 1 {
            self.limit = 1;
        }
        if self.limit > 500 {
            self.limit = 500;
        }

        if self.offset < 0 {
            self.offset = 0;
        }

        if let (Some(from), Some(to)) = (self.year_from, self.year_to) {
            if from > to {
                return Err("year_from cannot be greater than year_to".to_string());
            }
        }

        Ok(())
    }
}

/// Request to merge a secondary title into a primary title.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeTitlesRequest {
    pub confirm: bool,
}

/// Response from merging two titles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeTitlesResponse {
    pub success: bool,
    pub primary_title_id: String,
    pub volumes_moved: i64,
    pub secondary_title_deleted: bool,
    pub message: String,
}
