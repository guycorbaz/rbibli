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
pub use shared::models::borrowers::BorrowerGroup;
pub use shared::models::borrowers::Borrower;
pub use shared::models::borrowers::BorrowerWithGroup;

pub use shared::dtos::borrowers::CreateBorrowerGroupRequest;

pub use shared::dtos::borrowers::UpdateBorrowerGroupRequest;

pub use shared::dtos::borrowers::CreateBorrowerRequest;

pub use shared::dtos::borrowers::UpdateBorrowerRequest;
