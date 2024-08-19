use chrono::NaiveDate;
use error_mapper::TheResult;
use mysql_async::Conn;
use crate::modules::clients::db::DbClients;
use crate::utilities::datatypes::{ClientsIdType, DocumentType, LicensePlateType, PhoneNumberType};

mod db;
pub mod services;

pub struct Clients {
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

impl Clients {
    pub async fn select_by_license_plate(conn: &mut Conn, license_plate: &LicensePlateType) -> TheResult<Option<Clients>> {
        DbClients::select_by_license_plate(
            conn,
            license_plate
        ).await.map(|query| query.map(Clients::from))
    }
    pub fn get_id(&self) -> ClientsIdType {
        self.id
    }
}
