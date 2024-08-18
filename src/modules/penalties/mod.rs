use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use crate::utilities::datatypes::{ClientsIdType, LicensePlateType, PenaltiesIdType, RegisteredVehiclesIdType, ViolationLogsIdType};

mod db;
pub mod services;

struct Penalties {
    id: PenaltiesIdType,
    violation_logs_id: Option<ViolationLogsIdType>,
    clients_id: Option<ClientsIdType>,
    registered_vehicles_id: Option<RegisteredVehiclesIdType>,
    license_plate: LicensePlateType,
    penalty_amount: Decimal,
    reason: String,
    penalty_paid: bool,
    paid_at: Option<NaiveDateTime>
}
