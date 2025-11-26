//! API handlers module.
//!
//! This module exports the handler functions for the various API endpoints of the application.
//! Each submodule corresponds to a specific resource or feature, such as titles, authors, loans, etc.

pub mod titles;
pub mod locations;
pub mod authors;
pub mod publishers;
pub mod genres;
pub mod series;
pub mod volumes;
pub mod uploads;
pub mod isbn_lookup;
pub mod borrower_groups;
pub mod borrowers;
pub mod loans;
pub mod statistics;
