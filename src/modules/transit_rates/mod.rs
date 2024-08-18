use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use crate::utilities::datatypes::{BoothsIdType, TransitRatesIdType, VehicleTypesIdType};

mod db;

struct TransitRates {
    id: TransitRatesIdType,
    vehicle_types_id: VehicleTypesIdType,
    fee_amount: Decimal,
    booths_id: Option<BoothsIdType>,
    valid_from: NaiveDateTime,
    valid_until: Option<NaiveDateTime>
}
