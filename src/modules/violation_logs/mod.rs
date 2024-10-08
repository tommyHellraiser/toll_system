use crate::utilities::datatypes::{ClientsIdType, LicensePlateType, RegisteredVehiclesIdType, ViolationLogsIdType};

mod db;
pub mod services;

struct ViolationLogs {
    id: ViolationLogsIdType,
    clients_id: Option<ClientsIdType>,
    registered_vehicles_id: Option<RegisteredVehiclesIdType>,
    license_plate: LicensePlateType,
    description: Option<String>
}
