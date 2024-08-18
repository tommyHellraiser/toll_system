
mod tests {
    use crate::modules::vehicle_types::VehicleType;

    //  VEHICLE TYPE - TO STRING
    #[test]
    fn vehicle_type_to_string_bike() {
        let value = VehicleType::Bike;

        assert_eq!("Bike", &value.to_string())
    }

    #[test]
    fn vehicle_type_to_string_car() {
        let value = VehicleType::Car;

        assert_eq!("Car", &value.to_string())
    }

    #[test]
    fn vehicle_type_to_string_pickup() {
        let value = VehicleType::Pickup;

        assert_eq!("Pickup", &value.to_string())
    }

    #[test]
    fn vehicle_type_to_string_van() {
        let value = VehicleType::Van;

        assert_eq!("Van", &value.to_string())
    }

    #[test]
    fn vehicle_type_to_string_truck() {
        let value = VehicleType::Truck;

        assert_eq!("Truck", &value.to_string())
    }

    #[test]
    fn vehicle_type_to_string_trailer() {
        let value = VehicleType::Trailer;

        assert_eq!("Trailer", &value.to_string())
    }

    #[test]
    fn vehicle_type_to_string_service() {
        let value = VehicleType::Service;

        assert_eq!("Service", &value.to_string())
    }

    #[test]
    fn vehicle_type_to_string_bus() {
        let value = VehicleType::Bus;

        assert_eq!("Bus", &value.to_string())
    }

    #[test]
    fn vehicle_type_to_string_unknown() {
        let value = VehicleType::Unknown;

        assert_eq!("Unknown", &value.to_string())
    }
    
    
    //  VEHICLE TYPE - FROM STRING
    #[test]
    fn vehicle_type_from_string_bike() {
        let value = String::from("Bike");

        assert_eq!(VehicleType::Bike, VehicleType::from(value))
    }

    #[test]
    fn vehicle_type_from_string_car() {
        let value = String::from("Car");

        assert_eq!(VehicleType::Car, VehicleType::from(value))
    }

    #[test]
    fn vehicle_type_from_string_pickup() {
        let value = String::from("Pickup");

        assert_eq!(VehicleType::Pickup, VehicleType::from(value))
    }

    #[test]
    fn vehicle_type_from_string_van() {
        let value = String::from("Van");

        assert_eq!(VehicleType::Van, VehicleType::from(value))
    }

    #[test]
    fn vehicle_type_from_string_truck() {
        let value = String::from("Truck");

        assert_eq!(VehicleType::Truck, VehicleType::from(value))
    }

    #[test]
    fn vehicle_type_from_string_trailer() {
        let value = String::from("Trailer");

        assert_eq!(VehicleType::Trailer, VehicleType::from(value))
    }

    #[test]
    fn vehicle_type_from_string_service() {
        let value = String::from("Service");

        assert_eq!(VehicleType::Service, VehicleType::from(value))
    }

    #[test]
    fn vehicle_type_from_string_bus() {
        let value = String::from("Bus");

        assert_eq!(VehicleType::Bus, VehicleType::from(value))
    }

    #[test]
    fn vehicle_type_from_string_unknown() {
        let value = String::from("Unknown");

        assert_eq!(VehicleType::Unknown, VehicleType::from(value))
    }

    #[test]
    fn vehicle_type_from_string_unknown_2() {
        let value = String::from("asd3f54a654");

        assert_eq!(VehicleType::Unknown, VehicleType::from(value))
    }
}
