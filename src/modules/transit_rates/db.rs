use chrono::NaiveDateTime;
use mysql_async::prelude::FromRow;
use mysql_async::{FromRowError, Row};
use rust_decimal::Decimal;
use crate::get_value_from_row;
use crate::modules::transit_rates::TransitRates;
use crate::utilities::datatypes::{BoothsIdType, TransitRatesIdType, VehicleTypesIdType};

struct DbTransitRates {
    id: TransitRatesIdType,
    vehicle_types_id: VehicleTypesIdType,
    fee_amount: Decimal,
    booths_id: Option<BoothsIdType>,
    valid_from: NaiveDateTime,
    valid_until: Option<NaiveDateTime>
}

impl FromRow for DbTransitRates {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let table = "transit_rates";
        Self {
            id: get_value_from_row!(row, "ID", table, TransitRatesIdType),
            vehicle_types_id: get_value_from_row!(row, "vehicle_types_ID", table, VehicleTypesIdType),
            fee_amount: get_value_from_row!(row, "fee_amount", table, Decimal),
            booths_id: get_value_from_row!(row, "booths_ID", table, Option<BoothsIdType>),
            valid_from: get_value_from_row!(row, "valid_from", table, NaiveDateTime),
            valid_until: get_value_from_row!(row, "valid_until", table, Option<NaiveDateTime>)
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl From<DbTransitRates> for TransitRates {
    fn from(value: DbTransitRates) -> Self {
        Self {
            id: value.id,
            vehicle_types_id: value.vehicle_types_id,
            fee_amount: value.fee_amount,
            booths_id: value.booths_id,
            valid_from: value.valid_from,
            valid_until: value.valid_until
        }
    }
}
