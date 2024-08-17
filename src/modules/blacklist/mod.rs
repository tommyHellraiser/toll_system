use chrono::NaiveDateTime;
use crate::utilities::datatypes::{BlacklistIdType, ClientsIdType};

mod db;

struct Blacklist {
    id: BlacklistIdType,
    clients_id: ClientsIdType,
    reason: String,
    restriction_expiry: NaiveDateTime
}
