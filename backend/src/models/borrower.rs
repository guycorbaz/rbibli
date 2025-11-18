//! Borrower and borrower group models for the rbibli library management system.
//!
//! This module defines data structures for managing library members (borrowers)
//! and borrower groups with loan duration policies.
//!
//! # Borrower Management
//!
//! The borrower system is trust-based and designed for small-scale use (friends and family).
//! Features include:
//! - Contact information management (name, email, phone, address)
//! - Group-based loan duration policies
//! - Active loan count tracking
//! - Optional group membership (borrowers can exist without a group)
//!
//! # Borrower Groups
//!
//! Groups allow organizing borrowers with different loan policies:
//! - Each group defines a default loan duration in days
//! - Borrowers inherit their group's loan duration
//! - Default loan duration is 21 days if no group is assigned
//!
//! # Trust-Based System
//!
//! This is a simple, trust-based system without:
//! - Fines or penalties
//! - Account suspensions
//! - Complex access controls

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Borrower group defining loan duration policies.
///
/// Groups allow organizing borrowers with different lending rules.
/// For example, you might have:
/// - "Family" group with 30-day loans
/// - "Friends" group with 21-day loans
/// - "Colleagues" group with 14-day loans
///
/// # Fields
///
/// * `id` - Unique identifier (UUID)
/// * `name` - Group name (e.g., "Family", "Friends")
/// * `loan_duration_days` - Default loan duration for this group
/// * `description` - Optional description
/// * `created_at` - Record creation timestamp
/// * `updated_at` - Last modification timestamp
///
/// # Loan Duration
///
/// When a loan is created, the system uses the borrower's group loan duration
/// to calculate the due date. If the borrower has no group, a default of
/// 21 days is used.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BorrowerGroup {
    /// Unique group identifier
    #[sqlx(try_from = "String")]
    pub id: Uuid,
    /// Group name (e.g., "Family")
    pub name: String,
    /// Default loan duration in days for this group
    pub loan_duration_days: i32,
    /// Optional description
    pub description: Option<String>,
    /// Record creation timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// Library member who can borrow books.
///
/// Borrowers represent people who can check out volumes from the library.
/// The system is designed for personal use with friends and family, so
/// it's intentionally simple and trust-based.
///
/// # Fields
///
/// * `id` - Unique identifier (UUID)
/// * `name` - Borrower's full name (required)
/// * `email` - Email address (optional)
/// * `phone` - Phone number (optional)
/// * `address` - Street address (optional)
/// * `city` - City (optional)
/// * `zip` - Postal code (optional)
/// * `group_id` - Reference to borrower group (optional)
/// * `created_at` - Record creation timestamp
/// * `updated_at` - Last modification timestamp
///
/// # Contact Information
///
/// All contact fields except name are optional. This flexibility allows
/// for minimal data collection while still enabling communication about
/// overdue loans if needed.
///
/// # Group Membership
///
/// Borrowers can optionally belong to a group, which determines their
/// default loan duration. If no group is assigned, the system uses a
/// default duration of 21 days.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Borrower {
    /// Unique borrower identifier
    #[sqlx(try_from = "String")]
    pub id: Uuid,
    /// Borrower's full name (required)
    pub name: String,
    /// Email address (optional)
    pub email: Option<String>,
    /// Phone number (optional)
    pub phone: Option<String>,
    /// Street address (optional)
    pub address: Option<String>,
    /// City (optional)
    pub city: Option<String>,
    /// Postal/ZIP code (optional)
    pub zip: Option<String>,
    /// Reference to borrower group (optional)
    pub group_id: Option<String>,
    /// Record creation timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// Extended borrower information with group details and loan count.
///
/// This structure combines borrower data with their group information
/// and current loan statistics. Used for displaying borrowers in the UI.
///
/// # Fields
///
/// * `borrower` - Core borrower record (flattened into this struct)
/// * `group_name` - Name of the borrower's group (if assigned)
/// * `loan_duration_days` - Loan duration from group (if assigned)
/// * `active_loan_count` - Number of currently active loans
///
/// # Usage
///
/// Returned by the list borrowers endpoint to provide complete context
/// for each borrower, including their lending privileges and current
/// loan activity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowerWithGroup {
    /// Core borrower record (fields are flattened into this struct)
    #[serde(flatten)]
    pub borrower: Borrower,
    /// Name of the borrower's group (None if no group assigned)
    pub group_name: Option<String>,
    /// Loan duration in days from group (None if no group assigned)
    pub loan_duration_days: Option<i32>,
    /// Number of currently active loans for this borrower
    pub active_loan_count: i32,
}

/// Request payload for creating a new borrower group.
///
/// # Fields
///
/// * `name` - Group name (required)
/// * `loan_duration_days` - Default loan duration for the group (required)
/// * `description` - Optional description
///
/// # Example
///
/// ```json
/// {
///   "name": "Family",
///   "loan_duration_days": 30,
///   "description": "Family members with extended loan periods"
/// }
/// ```
///
/// # Validation
///
/// - Name must not be empty
/// - Loan duration must be positive (typically 7-90 days)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBorrowerGroupRequest {
    /// Group name (required)
    pub name: String,
    /// Default loan duration in days (required, typically 7-90)
    pub loan_duration_days: i32,
    /// Optional description
    pub description: Option<String>,
}

/// Request payload for updating an existing borrower group.
///
/// All fields are optional, allowing partial updates.
///
/// # Fields
///
/// * `name` - Optional new group name
/// * `loan_duration_days` - Optional new loan duration
/// * `description` - Optional new description
///
/// # Behavior
///
/// Only provided fields will be updated. Changing a group's loan duration
/// does NOT affect existing loans, only future loans created for borrowers
/// in this group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBorrowerGroupRequest {
    /// Optional new group name
    pub name: Option<String>,
    /// Optional new loan duration in days
    pub loan_duration_days: Option<i32>,
    /// Optional new description
    pub description: Option<String>,
}

/// Request payload for creating a new borrower.
///
/// # Fields
///
/// * `name` - Borrower's full name (required)
/// * `email` - Email address (optional)
/// * `phone` - Phone number (optional)
/// * `address` - Street address (optional)
/// * `city` - City (optional)
/// * `zip` - Postal code (optional)
/// * `group_id` - UUID of borrower group (optional)
///
/// # Example
///
/// ```json
/// {
///   "name": "John Doe",
///   "email": "john@example.com",
///   "phone": "+1-555-0123",
///   "group_id": "group-uuid-here"
/// }
/// ```
///
/// # Validation
///
/// - Name must not be empty
/// - If group_id is provided, the group must exist
/// - Email format is not strictly validated (trust-based system)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBorrowerRequest {
    /// Borrower's full name (required)
    pub name: String,
    /// Email address (optional)
    pub email: Option<String>,
    /// Phone number (optional)
    pub phone: Option<String>,
    /// Street address (optional)
    pub address: Option<String>,
    /// City (optional)
    pub city: Option<String>,
    /// Postal/ZIP code (optional)
    pub zip: Option<String>,
    /// UUID of borrower group (optional)
    pub group_id: Option<String>,
}

/// Request payload for updating an existing borrower.
///
/// All fields are optional, allowing partial updates. Only provided
/// fields will be updated in the database.
///
/// # Fields
///
/// * `name` - Optional new name
/// * `email` - Optional new email
/// * `phone` - Optional new phone
/// * `address` - Optional new address
/// * `city` - Optional new city
/// * `zip` - Optional new postal code
/// * `group_id` - Optional new group assignment
///
/// # Behavior
///
/// - To clear an optional field, explicitly set it to null
/// - Changing group_id does NOT affect existing loans
/// - If group_id is changed, future loans will use the new group's duration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBorrowerRequest {
    /// Optional new name
    pub name: Option<String>,
    /// Optional new email
    pub email: Option<String>,
    /// Optional new phone
    pub phone: Option<String>,
    /// Optional new address
    pub address: Option<String>,
    /// Optional new city
    pub city: Option<String>,
    /// Optional new postal code
    pub zip: Option<String>,
    /// Optional new group assignment
    pub group_id: Option<String>,
}
