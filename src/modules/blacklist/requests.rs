use chrono::NaiveDateTime;
use serde::Deserialize;
use crate::utilities::datatypes::{BlacklistIdType, ClientsIdType, LicensePlateType};

#[derive(Deserialize)]
pub(super) struct ClientPath {
    pub(super) clients_id: ClientsIdType
}

#[derive(Deserialize)]
pub(super) struct BlacklistPath {
    pub(super) blacklist_id: BlacklistIdType
}

#[derive(Deserialize, Clone)]
pub struct BlacklistPost {
    pub clients_id: Option<ClientsIdType>,
    pub license_plate: LicensePlateType,
    pub reason: String,
    pub restriction_expiry: Option<NaiveDateTime>
}

impl BlacklistPost {
    pub fn validate(&self) -> Vec<String> {
        let mut errors = vec![];
        
        if self.license_plate.is_empty() {
            errors.push("License plate field cannot be empty".to_string());
        }
        
        if self.reason.is_empty() {
            errors.push("Blacklist reason cannot be empty".to_string());
        }
        
        if let Some(expiry) = self.restriction_expiry {
            if expiry < chrono::Local::now().naive_local() {
                errors.push("Expiration date cannot be anterior to current date".to_string());
            }
        }
        
        errors
    }
}
