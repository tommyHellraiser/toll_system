use chrono::NaiveDate;
use crate::utilities::datatypes::{ClientsIdType, DocumentType, PhoneNumberType};

mod db;

struct Clients {
    id: ClientsIdType,
    first_name: String,
    last_name: String,
    document: DocumentType,
    email: String,
    date_of_birth: NaiveDate,
    phone_number: Option<PhoneNumberType>,
    address: Option<String>,
    is_active: bool
}
