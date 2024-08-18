use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use crate::utilities::datatypes::{BoothsIdType, DiscountsIdType, VehicleTypesIdType};

mod db;
pub mod services;

struct Discounts {
    id: DiscountsIdType,
    vehicle_types_id: VehicleTypesIdType,
    valid_from: NaiveDateTime,
    valid_until: NaiveDateTime,
    discount_percentage: Decimal,
    booths_id: BoothsIdType,
    description: String
}