use chrono::NaiveDateTime;
use mysql_async::prelude::FromRow;
use mysql_async::{FromRowError, Row};
use rust_decimal::Decimal;
use crate::get_value_from_row;
use crate::modules::penalties::Penalties;
use crate::utilities::datatypes::{ClientsIdType, LicensePlateType, PenaltiesIdType, RegisteredVehiclesIdType, ViolationLogsIdType};

struct DbPenalties {
    id: PenaltiesIdType,
    violation_logs_id: ViolationLogsIdType,
    clients_id: ClientsIdType,
    registered_vehicles_id: RegisteredVehiclesIdType,
    license_plate: LicensePlateType,
    penalty_amount: Decimal,
    reason: String,
    penalty_paid: bool,
    paid_at: NaiveDateTime
}

impl FromRow for DbPenalties {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let table = "penalties";
        Self {
            id: get_value_from_row!(row, "ID", table, PenaltiesIdType),
            violation_logs_id: get_value_from_row!(row, "violation_logs_ID", table, ViolationLogsIdType),
            clients_id: get_value_from_row!(row, "clients_ID", table, ClientsIdType),
            registered_vehicles_id: get_value_from_row!(row, "registered_vehicles_ID", table, RegisteredVehiclesIdType),
            license_plate: get_value_from_row!(row, "license_plate", table, LicensePlateType),
            penalty_amount: get_value_from_row!(row, "penalty_amount", table, Decimal),
            reason: get_value_from_row!(row, "reason", table, String),
            penalty_paid: get_value_from_row!(row, "penalty_paid", table, bool),
            paid_at: get_value_from_row!(row, "paid_at", table, NaiveDateTime)
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl From<DbPenalties> for Penalties {
    fn from(value: DbPenalties) -> Self {
        Self {
            id: value.id,
            violation_logs_id: value.violation_logs_id,
            clients_id: value.clients_id,
            registered_vehicles_id: value.registered_vehicles_id,
            license_plate: value.license_plate,
            penalty_amount: value.penalty_amount,
            reason: value.reason,
            penalty_paid: value.penalty_paid,
            paid_at: value.paid_at
        }
    }
}
