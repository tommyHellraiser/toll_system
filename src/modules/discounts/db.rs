use chrono::{NaiveDateTime};
use mysql_async::prelude::FromRow;
use mysql_async::{FromRowError, Row};
use rust_decimal::Decimal;
use crate::get_value_from_row;
use crate::modules::discounts::Discounts;
use crate::utilities::datatypes::{BoothsIdType, DiscountsIdType, VehicleTypesIdType};

struct DbDiscounts {
    id: DiscountsIdType,
    vehicle_types_id: VehicleTypesIdType,
    valid_from: NaiveDateTime,
    valid_until: NaiveDateTime,
    discount_percentage: Decimal,
    booths_id: BoothsIdType,
    description: String
}

impl FromRow for DbDiscounts {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let table = "discounts";
        Self {
            id: get_value_from_row!(row, "ID", table, DiscountsIdType),
            vehicle_types_id: get_value_from_row!(row, "ID", table, VehicleTypesIdType),
            valid_from: get_value_from_row!(row, "ID", table, NaiveDateTime),
            valid_until: get_value_from_row!(row, "ID", table, NaiveDateTime),
            discount_percentage: get_value_from_row!(row, "ID", table, Decimal),
            booths_id: get_value_from_row!(row, "ID", table, BoothsIdType),
            description: get_value_from_row!(row, "ID", table, String)
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl From<DbDiscounts> for Discounts {
    fn from(value: DbDiscounts) -> Self {
        Self {
            id: value.id,
            vehicle_types_id: value.vehicle_types_id,
            valid_from: value.valid_from,
            valid_until: value.valid_until,
            discount_percentage: value.discount_percentage,
            booths_id: value.booths_id,
            description: value.description
        }
    }
}
