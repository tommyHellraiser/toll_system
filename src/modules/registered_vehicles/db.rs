use mysql_async::prelude::FromRow;
use mysql_async::{FromRowError, Row};
use crate::get_value_from_row;
use crate::modules::registered_vehicles::RegisteredVehicles;
use crate::utilities::datatypes::{ClientsIdType, LicensePlateType, RegisteredVehiclesIdType, VehicleTypesIdType};

struct DbRegisteredVehicles {
    id: RegisteredVehiclesIdType,
    clients_id: ClientsIdType,
    vehicle_types_id: VehicleTypesIdType,
    license_plate: LicensePlateType,
    color: String,
    brand: String,
    model: String,
    year: u16
}

impl FromRow for DbRegisteredVehicles {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let table = "registered_vehicles";
        Self {
            id: get_value_from_row!(row, "ID", table, RegisteredVehiclesIdType),
            clients_id: get_value_from_row!(row, "clients_ID", table, ClientsIdType),
            vehicle_types_id: get_value_from_row!(row, "vehicle_types_ID", table, VehicleTypesIdType),
            license_plate: get_value_from_row!(row, "license_plate", table, LicensePlateType),
            color: get_value_from_row!(row, "color", table, String),
            brand: get_value_from_row!(row, "brand", table, String),
            model: get_value_from_row!(row, "model", table, String),
            year: get_value_from_row!(row, "year", table, u16)
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl From<DbRegisteredVehicles> for RegisteredVehicles {
    fn from(value: DbRegisteredVehicles) -> Self {
        Self {
            id: value.id,
            clients_id: value.clients_id,
            vehicle_types_id: value.vehicle_types_id,
            license_plate: value.license_plate,
            color: value.color,
            brand: value.brand,
            model: value.model,
            year: value.year
        }
    }
}
