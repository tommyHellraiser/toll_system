use chrono::{NaiveDateTime};
use error_mapper::{create_new_error, TheResult};
use mysql_async::prelude::{FromRow, Queryable};
use mysql_async::{Conn, FromRowError, Row};
use crate::{get_value_from_row, DATETIME_FORMAT};
use crate::modules::blacklist::Blacklist;
use crate::modules::blacklist::requests::{BlacklistPatch, BlacklistPost};
use crate::utilities::datatypes::{BlacklistIdType, ClientsIdType, LicensePlateType};

#[derive(Default, Debug)]
pub(super) struct DbBlacklist {
    id: BlacklistIdType,
    clients_id: Option<ClientsIdType>,
    license_plate: LicensePlateType,
    reason: String,
    restriction_expiry: Option<NaiveDateTime>
}

impl DbBlacklist {
    pub(super) async fn select_open_blacklist(conn: &mut Conn) -> TheResult<Vec<DbBlacklist>> {

        let query = "SELECT * \
        FROM blacklist \
        WHERE restriction_expiry > NOW() \
        OR restriction_expiry IS NULL;";

        conn.query::<DbBlacklist, _>(query)
            .await
            .map_err(|error| create_new_error!(error))
    }

    pub(super) async fn select_by_clients_id(
        conn: &mut Conn,
        clients_id: ClientsIdType
    ) -> TheResult<Vec<DbBlacklist>> {

        let query = "SELECT * FROM blacklist \
        WHERE clients_ID = ? \
        ORDER BY ID DESC;";
        let params = vec![Some(clients_id)];

        conn.exec(query, params)
            .await
            .map_err(|error| create_new_error!(error))

    }

    pub(super) async fn select_by_clients_id_or_license_plate(
        conn: &mut Conn,
        clients_id: Option<ClientsIdType>,
        license_plate: &LicensePlateType
    ) -> TheResult<Vec<DbBlacklist>> {

        let mut query = "SELECT * FROM blacklist \
        WHERE license_plate = ? ".to_string();
        let mut params = vec![license_plate.to_string()];

        if let Some(clients_id) = clients_id {
            query.push_str("OR clients_id = ? ");
            params.push(clients_id.to_string());
        }

        query.push_str("AND (restriction_expiry > NOW() OR restriction_expiry IS NULL);");

        conn.exec(query, params).await.map_err(|error| create_new_error!(error))
    }

    pub(super) async fn insert_new_entry(&mut self, conn: &mut Conn) -> TheResult<bool> {

        let query = "INSERT INTO blacklist (clients_ID, license_plate, reason, restriction_expiry) \
        VALUES(?, ?, ?, ?);";
        let params = vec![
            self.clients_id.map(|id| id.to_string()),
            Some(self.license_plate.clone()),
            Some(self.reason.clone()),
            self.restriction_expiry.map(|expiry| expiry.to_string())
        ];

        conn.exec_drop(query, params).await.map_err(|error| create_new_error!(error))?;

        self.id = conn.last_insert_id().unwrap_or_default() as u32;

        Ok(conn.affected_rows() > 0)
    }

    pub(super) async fn update_by_id(
        conn: &mut Conn,
        id: BlacklistIdType,
        patch_request: BlacklistPatch
    ) -> TheResult<bool> {

        let mut query = "UPDATE blacklist SET ".to_string();
        let mut params = vec![];
        
        if let Some(reason) = patch_request.reason {
            query.push_str("reason = ?,");
            params.push(reason);
        }
        
        if let Some(expiry) = patch_request.restriction_expiry {
            query.push_str("restriction_expiry = ?,");
            params.push(expiry.format(DATETIME_FORMAT).to_string());
        }
        
        query.pop();    // Remove trailing comma
        query.push_str(" WHERE ID = ?");
        params.push(id.to_string());
        
        conn.exec_drop(query, params).await.map_err(|error| create_new_error!(error))?;

        let affected_rows = conn.affected_rows();
        
        Ok(affected_rows > 0)
    }
}

impl FromRow for DbBlacklist {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let table = "blacklist";
        Self {
            id: get_value_from_row!(row, "ID", table, BlacklistIdType),
            clients_id: get_value_from_row!(row, "clients_ID", table, Option<ClientsIdType>),
            license_plate: get_value_from_row!(row, "license_plate", table, LicensePlateType),
            reason: get_value_from_row!(row, "reason", table, String),
            restriction_expiry: get_value_from_row!(row, "restriction_expiry", table, Option<NaiveDateTime>)
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl From<DbBlacklist> for Blacklist {
    fn from(value: DbBlacklist) -> Self {
        Self {
            id: value.id,
            clients_id: value.clients_id,
            license_plate: value.license_plate,
            reason: value.reason,
            restriction_expiry: value.restriction_expiry
        }
    }
}

impl From<BlacklistPost> for DbBlacklist {
    fn from(value: BlacklistPost) -> Self {
        Self {
            id: 0,
            clients_id: value.clients_id,
            license_plate: value.license_plate,
            reason: value.reason,
            restriction_expiry: value.restriction_expiry
        }
    }
}
