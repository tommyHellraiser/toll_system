use std::fmt::{Display, Formatter};
use crate::utilities::datatypes::VehicleTypesIdType;

mod db;

#[cfg(test)]
mod tests;

struct VehicleTypes {
    id: VehicleTypesIdType,
    vehicle_type: VehicleType
}

#[derive(PartialOrd, PartialEq, Debug)]
enum VehicleType {
    Bike,
    Car,
    Pickup,
    Van,
    Truck,
    Trailer,
    Service,
    Bus,
    Unknown
}

impl Display for VehicleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            VehicleType::Bike => { "Bike" },
            VehicleType::Car => { "Car" },
            VehicleType::Pickup => { "Pickup" },
            VehicleType::Van => { "Van" },
            VehicleType::Truck => { "Truck" },
            VehicleType::Trailer => { "Trailer" },
            VehicleType::Service => { "Service" },
            VehicleType::Bus => { "Bus" },
            VehicleType::Unknown => { "Unknown" },
        };

        write!(f, "{}", content)
    }
}

impl From<String> for VehicleType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Bike" => { VehicleType::Bike },
            "Car" => { VehicleType::Car },
            "Pickup" => { VehicleType::Pickup },
            "Van" => { VehicleType::Van },
            "Truck" => { VehicleType::Truck },
            "Trailer" => { VehicleType::Trailer },
            "Service" => { VehicleType::Service },
            "Bus" => { VehicleType::Bus },
            _ => { VehicleType::Unknown },
        }
    }
}
