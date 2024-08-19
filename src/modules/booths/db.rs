use mysql_async::prelude::FromRow;
use mysql_async::{FromRowError, Row};
use crate::get_value_from_row;
use crate::utilities::datatypes::BoothsIdType;

struct DbBooths {
    id: BoothsIdType,
    city: String,
    route: String,
    max_lanes: u8,
    is_active: bool
}

impl FromRow for DbBooths {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let table = "booths";
        Self {
            id: get_value_from_row!(row, "ID", table, BoothsIdType),
            city: get_value_from_row!(row, "city", table, String),
            route: get_value_from_row!(row, "route", table, String),
            max_lanes: get_value_from_row!(row, "max_lanes", table, u8),
            is_active: get_value_from_row!(row, "is_active", table, bool)
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
