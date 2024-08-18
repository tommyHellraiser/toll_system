use mysql_async::prelude::FromRow;
use mysql_async::{FromRowError, Row};
use crate::get_value_from_row;
use crate::modules::vehicle_types::{VehicleType, VehicleTypes};
use crate::utilities::datatypes::VehicleTypesIdType;

struct DbVehicleTypes {
    id: VehicleTypesIdType,
    vehicle_type: VehicleType
}

impl FromRow for DbVehicleTypes {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let table = "vehicle_types";
        Self {
            id: get_value_from_row!(row, "ID", table, VehicleTypesIdType),
            vehicle_type: get_value_from_row!(row, "vehicle_type", table, String).into()
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl From<DbVehicleTypes> for VehicleTypes {
    fn from(value: DbVehicleTypes) -> Self {
        Self {
            id: value.id,
            vehicle_type: value.vehicle_type
        }
    }
}
