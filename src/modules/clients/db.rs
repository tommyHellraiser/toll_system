use chrono::NaiveDate;
use error_mapper::{create_new_error, TheResult};
use mysql_async::prelude::{FromRow, Queryable};
use mysql_async::{Conn, FromRowError, Row};
use crate::get_value_from_row;
use crate::modules::clients::Clients;
use crate::utilities::datatypes::{ClientsIdType, DocumentType, LicensePlateType, PhoneNumberType};

pub(super) struct DbClients {
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

impl DbClients {
    pub(super) async fn select_by_license_plate(
        conn: &mut Conn,
        license_plate: &LicensePlateType
    ) -> TheResult<Option<DbClients>> {
        
        let query = "SELECT c.* \
        FROM clients AS c \
        LEFT JOIN registered_vehicles AS r \
        ON r.clients_ID = c.ID \
        WHERE license_plate = ?;";
        let params = vec![license_plate];
        
        conn.exec_first(query, params)
            .await
            .map_err(|error| create_new_error!(error))
    }
}

impl FromRow for DbClients {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let table = "clients";
        Self {
            id: get_value_from_row!(row, "ID", table, ClientsIdType),
            first_name: get_value_from_row!(row, "first_name", table, String),
            last_name: get_value_from_row!(row, "last_name", table, String),
            document: get_value_from_row!(row, "document", table, DocumentType),
            email: get_value_from_row!(row, "email", table, String),
            date_of_birth: get_value_from_row!(row, "date_of_birth", table, NaiveDate),
            phone_number: get_value_from_row!(row, "phone_number", table, Option<PhoneNumberType>),
            address: get_value_from_row!(row, "address", table, Option<String>),
            is_active: get_value_from_row!(row, "is_active", table, bool)
        }
    }
    
    fn from_row_opt(_row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl From<DbClients> for Clients {
    fn from(value: DbClients) -> Self {
        Self {
            id: value.id,
            first_name: value.first_name,
            last_name: value.last_name,
            document: value.document,
            email: value.email,
            date_of_birth: value.date_of_birth,
            phone_number: value.phone_number,
            address: value.address,
            is_active: value.is_active
        }
    }
}
