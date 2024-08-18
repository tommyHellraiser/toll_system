use chrono::NaiveDateTime;
use crate::utilities::datatypes::{BlacklistIdType, ClientsIdType, LicensePlateType};

mod db;

struct Blacklist {
    id: BlacklistIdType,
    clients_id: Option<ClientsIdType>,
    license_plate: LicensePlateType,
    reason: String,
    restriction_expiry: Option<NaiveDateTime>
}
