//! Location management models.
//!
//! This module defines the data structures for managing physical storage locations.
//! Locations are hierarchical, allowing for structures like "Room A > Shelf 1 > Bin 3".
//!
//! # Key Features
//!
//! - **Hierarchy**: Supports parent-child relationships for nested locations.
//! - **Path Generation**: Can generate full paths (e.g., "Library > Fiction > A-M").
//! - **Statistics**: Tracks volume counts per location.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Location represents a physical place where volumes can be stored.
///
/// Locations can be nested to create a hierarchy (e.g., Building -> Room -> Shelf).
///
/// # Database Structure
///
/// Mapped to the `locations` table in the database.
pub use shared::models::locations::Location;
pub use shared::models::locations::LocationWithPath;

pub use shared::dtos::locations::CreateLocationRequest;

pub use shared::dtos::locations::UpdateLocationRequest;
