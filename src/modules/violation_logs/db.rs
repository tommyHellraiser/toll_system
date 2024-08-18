use mysql_async::prelude::FromRow;
use mysql_async::{FromRowError, Row};
use crate::get_value_from_row;
use crate::utilities::datatypes::{ClientsIdType, LicensePlateType, RegisteredVehiclesIdType, ViolationLogsIdType};

struct DbViolationLogs {
    id: ViolationLogsIdType,
    clients_id: Option<ClientsIdType>,
    registered_vehicles_id: Option<RegisteredVehiclesIdType>,
    license_plate: LicensePlateType,
    description: Option<String>
}

impl FromRow for DbViolationLogs {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let table = "violation_logs";
        Self {
            id: get_value_from_row!(row, "ID", table, ViolationLogsIdType),
            clients_id: get_value_from_row!(row, "clients_ID", table, Option<ClientsIdType>),
            registered_vehicles_id: get_value_from_row!(row, "registered_vehicles_ID", table, Option<RegisteredVehiclesIdType>),
            license_plate: get_value_from_row!(row, "license_plate", table, LicensePlateType),
            description: get_value_from_row!(row, "description", table, Option<String>)
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
