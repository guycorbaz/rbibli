//! Data models module.
//!
//! This module exports the data structures used throughout the application, including
//! database entities and API request/response objects.

pub mod title;
pub mod location;
pub mod author;
pub mod publisher;
pub mod genre;
pub mod series;
pub mod volume;
pub mod borrower;
pub mod loan;


// Re-export shared enums
pub use shared::models::enums::{
    VolumeCondition,
    LoanStatus as VolumeLoanStatus,
    AuthorRole,
    DuplicateConfidence,
    LoanRecordStatus as LoanStatus,
};


pub use title::{
    Title, TitleWithCount, CreateTitleRequest, UpdateTitleRequest, TitleSearchParams,
    DuplicatePair, DuplicateDetectionResponse, MergeTitlesRequest, MergeTitlesResponse,
};
pub use location::{Location, LocationWithPath, CreateLocationRequest, UpdateLocationRequest};
pub use author::{
    Author, AuthorWithTitleCount, CreateAuthorRequest, UpdateAuthorRequest,
    TitleAuthor, AddAuthorToTitleRequest
};
pub use publisher::{
    Publisher, PublisherWithTitleCount, CreatePublisherRequest, UpdatePublisherRequest
};
pub use genre::{
    Genre, GenreWithTitleCount, CreateGenreRequest, UpdateGenreRequest
};
pub use series::{
    Series, SeriesWithTitleCount, CreateSeriesRequest, UpdateSeriesRequest
};
pub use volume::{
    Volume, CreateVolumeRequest, UpdateVolumeRequest
};
pub use borrower::{
    BorrowerGroup, Borrower, BorrowerWithGroup,
    CreateBorrowerGroupRequest, UpdateBorrowerGroupRequest,
    CreateBorrowerRequest, UpdateBorrowerRequest
};
pub use loan::{
    Loan, LoanDetail, CreateLoanRequest, ReturnLoanRequest
};

