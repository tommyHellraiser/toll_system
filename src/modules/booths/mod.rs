use crate::utilities::datatypes::BoothsIdType;

mod db;
pub mod services;

struct Booths {
    id: BoothsIdType,
    city: String,
    route: String,
    max_lanes: u8,
    is_active: bool
}
