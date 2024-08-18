use chrono::NaiveDateTime;
use crate::utilities::datatypes::{ClientsIdType, LicensePlateType, RegisteredVehiclesIdType, VehicleTypesIdType};

mod db;

struct RegisteredVehicles {
    id: RegisteredVehiclesIdType,
    clients_id: ClientsIdType,
    vehicle_types_id: VehicleTypesIdType,
    license_plate: LicensePlateType,
    color: String,
    brand: String,
    model: String,
    year: u16,
    active_until: Option<NaiveDateTime>
}