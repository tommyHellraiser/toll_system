use chrono::NaiveDateTime;
use crate::utilities::datatypes::{BoothsIdType, ClientsIdType, LicensePlateType, RegisteredVehiclesIdType, TransitFeeLogsIdType, TransitLogsIdType, ViolationLogsIdType};

mod db;

struct TransitLogs {
    id: TransitLogsIdType,
    booths_id: BoothsIdType,
    clients_id: Option<ClientsIdType>,
    registered_vehicles_id: Option<RegisteredVehiclesIdType>,
    license_plate: LicensePlateType,
    transit_time: NaiveDateTime,
    transit_fee_logs_id: TransitFeeLogsIdType,
    violation_logs_id: Option<ViolationLogsIdType>
}
