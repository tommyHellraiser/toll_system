use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use crate::utilities::datatypes::{ClientsIdType, LicensePlateType, PenaltiesIdType, RegisteredVehiclesIdType, ViolationLogsIdType};

mod db;

struct Penalties {
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
