use chrono::NaiveDateTime;
use mysql_async::prelude::FromRow;
use mysql_async::{FromRowError, Row};
use crate::get_value_from_row;
use crate::modules::transit_logs::TransitLogs;
use crate::utilities::datatypes::{BoothsIdType, ClientsIdType, LicensePlateType, RegisteredVehiclesIdType, TransitFeeLogsIdType, TransitLogsIdType, ViolationLogsIdType};

struct DbTransitLogs {
    id: TransitLogsIdType,
    booths_id: BoothsIdType,
    clients_id: Option<ClientsIdType>,
    registered_vehicles_id: Option<RegisteredVehiclesIdType>,
    license_plate: LicensePlateType,
    transit_time: NaiveDateTime,
    transit_fee_logs_id: TransitFeeLogsIdType,
    violation_logs_id: Option<ViolationLogsIdType>
}

impl FromRow for DbTransitLogs {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let table = "transit_logs";
        Self {
            id: get_value_from_row!(row, "ID", table, TransitLogsIdType),
            booths_id: get_value_from_row!(row, "booths_ID", table, BoothsIdType),
            clients_id: get_value_from_row!(row, "clients_ID", table, Option<ClientsIdType>),
            registered_vehicles_id: get_value_from_row!(row, "registered_vehicles_ID", table, Option<RegisteredVehiclesIdType>),
            license_plate: get_value_from_row!(row, "license_plate", table, LicensePlateType),
            transit_time: get_value_from_row!(row, "transit_time", table, NaiveDateTime),
            transit_fee_logs_id: get_value_from_row!(row, "transit_fee_logs_ID", table, TransitFeeLogsIdType),
            violation_logs_id: get_value_from_row!(row, "violation_logs_ID", table, Option<ViolationLogsIdType>)
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl From<DbTransitLogs> for TransitLogs {
    fn from(value: DbTransitLogs) -> Self {
        Self {
            id: value.id,
            booths_id: value.booths_id,
            clients_id: value.clients_id,
            registered_vehicles_id: value.registered_vehicles_id,
            license_plate: value.license_plate,
            transit_time: value.transit_time,
            transit_fee_logs_id: value.transit_fee_logs_id,
            violation_logs_id: value.violation_logs_id
        }
    }
}
