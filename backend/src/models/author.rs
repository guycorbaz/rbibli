use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Author represents a book author
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub biography: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub death_date: Option<NaiveDate>,
    pub nationality: Option<String>,
    pub website_url: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// AuthorWithTitleCount includes the number of titles by this author
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorWithTitleCount {
    #[serde(flatten)]
    pub author: Author,
    pub title_count: i64,
}

/// CreateAuthorRequest for creating a new author
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuthorRequest {
    pub first_name: String,
    pub last_name: String,
    pub biography: Option<String>,
    pub birth_date: Option<String>,  // ISO format: YYYY-MM-DD
    pub death_date: Option<String>,  // ISO format: YYYY-MM-DD
    pub nationality: Option<String>,
    pub website_url: Option<String>,
}

/// UpdateAuthorRequest for updating an author
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAuthorRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub biography: Option<String>,
    pub birth_date: Option<String>,  // ISO format: YYYY-MM-DD
    pub death_date: Option<String>,  // ISO format: YYYY-MM-DD
    pub nationality: Option<String>,
    pub website_url: Option<String>,
}

/// TitleAuthor represents the relationship between a title and an author
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleAuthor {
    pub id: Uuid,
    pub title_id: Uuid,
    pub author_id: Uuid,
    pub role: AuthorRole,
    pub display_order: i32,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

/// Author roles in a title
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AuthorRole {
    MainAuthor,
    CoAuthor,
    Translator,
    Illustrator,
    Editor,
}

impl std::fmt::Display for AuthorRole {
    /// Formats the AuthorRole enum as a snake_case string for display and serialization.
    ///
    /// This implementation converts each AuthorRole variant into its corresponding
    /// snake_case string representation, which is used for database storage and
    /// API responses.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write the string representation to
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Successfully formatted the role
    /// * `Err(std::fmt::Error)` - Failed to write to the formatter
    ///
    /// # Examples
    ///
    /// ```
    /// use backend::models::AuthorRole;
    ///
    /// assert_eq!(AuthorRole::MainAuthor.to_string(), "main_author");
    /// assert_eq!(AuthorRole::CoAuthor.to_string(), "co_author");
    /// assert_eq!(AuthorRole::Translator.to_string(), "translator");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthorRole::MainAuthor => write!(f, "main_author"),
            AuthorRole::CoAuthor => write!(f, "co_author"),
            AuthorRole::Translator => write!(f, "translator"),
            AuthorRole::Illustrator => write!(f, "illustrator"),
            AuthorRole::Editor => write!(f, "editor"),
        }
    }
}

/// AddAuthorToTitleRequest for associating an author with a title
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddAuthorToTitleRequest {
    pub author_id: String,
    pub role: AuthorRole,
    pub display_order: Option<i32>,
}
