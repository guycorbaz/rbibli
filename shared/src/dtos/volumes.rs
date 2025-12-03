use serde::{Deserialize, Serialize};
use crate::models::enums::{VolumeCondition, LoanStatus};

/// Request payload for creating a new volume.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVolumeRequest {
    pub title_id: String,
    pub barcode: String,
    pub condition: VolumeCondition,
    pub location_id: Option<String>,
    pub individual_notes: Option<String>,
}

/// Request payload for updating an existing volume.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVolumeRequest {
    pub barcode: Option<String>,
    pub condition: Option<VolumeCondition>,
    pub location_id: Option<String>,
    pub loan_status: Option<LoanStatus>,
    pub individual_notes: Option<String>,
}
