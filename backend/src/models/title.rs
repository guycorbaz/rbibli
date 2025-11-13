use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Title represents the abstract book metadata shared across all physical copies
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Title {
    #[sqlx(try_from = "String")]
    pub id: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub isbn: Option<String>,
    pub publisher: Option<String>,
    pub publisher_id: Option<String>,
    pub publication_year: Option<i32>,
    pub pages: Option<i32>,
    pub language: String,
    pub dewey_code: Option<String>,
    pub dewey_category: Option<String>,
    pub genre: Option<String>,
    pub genre_id: Option<String>,
    pub summary: Option<String>,
    pub cover_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "base64_option")]
    pub image_data: Option<Vec<u8>>,
    pub image_mime_type: Option<String>,
    pub image_filename: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// Helper module for base64 encoding/decoding of image data
mod base64_option {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(data: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match data {
            Some(bytes) => {
                use base64::{engine::general_purpose::STANDARD, Engine};
                serializer.serialize_str(&STANDARD.encode(bytes))
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use base64::{engine::general_purpose::STANDARD, Engine};
        let opt: Option<String> = Option::deserialize(deserializer)?;
        match opt {
            Some(s) => STANDARD
                .decode(&s)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

/// TitleWithCount is returned by the list endpoint, includes volume count
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleWithCount {
    #[serde(flatten)]
    pub title: Title,
    pub volume_count: i64,
}

/// CreateTitleRequest for creating a new title
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
    pub dewey_category: Option<String>,
    #[serde(alias = "genre")]
    pub genre_id: Option<String>,
    pub summary: Option<String>,
    pub cover_url: Option<String>,
}

/// UpdateTitleRequest for updating an existing title
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
    pub dewey_category: Option<String>,
    #[serde(alias = "genre")]
    pub genre_id: Option<String>,
    pub summary: Option<String>,
    pub cover_url: Option<String>,
}
