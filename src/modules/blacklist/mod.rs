use chrono::NaiveDateTime;
use error_mapper::{create_new_error, TheResult};
use mysql_async::Conn;
use serde::Serialize;
use the_logger::{log_error, TheLogger};
use crate::modules::blacklist::db::DbBlacklist;
use crate::modules::blacklist::requests::{BlacklistPatch, BlacklistPost};
use crate::utilities::datatypes::{BlacklistIdType, ClientsIdType, LicensePlateType};

mod db;
pub mod services;
mod requests;
mod process;

#[derive(Serialize)]
pub struct Blacklist {
    id: BlacklistIdType,
    clients_id: Option<ClientsIdType>,
    license_plate: LicensePlateType,
    reason: String,
    restriction_expiry: Option<NaiveDateTime>
}

impl Blacklist {
    pub async fn select_open_blacklist(conn: &mut Conn) -> TheResult<Vec<Blacklist>> {
        let result = DbBlacklist::select_open_blacklist(conn)
            .await
            .map_err(|error| create_new_error!(error))?
            .into_iter()
            .map(Blacklist::from)
            .collect::<Vec<Blacklist>>();
        
        Ok(result)
    }
    
    pub async fn select_by_clients_id(
        conn: &mut Conn,
        clients_id: ClientsIdType
    ) -> TheResult<Option<Blacklist>> {
        
        let mut result = DbBlacklist::select_by_clients_id(
            conn,
            clients_id
        ).await?;
        
        if result.len() > 1 {
            return Err(create_new_error!(
                format!("More than one active entry was found for client ID: {}", clients_id)
            ))
        }
        
        Ok(result.pop().map(Blacklist::from))
    }

    pub async fn select_by_license_plate(
        conn: &mut Conn,
        license_plate: &LicensePlateType
    ) -> TheResult<Option<Blacklist>> {

        let mut result = DbBlacklist::select_by_clients_id_or_license_plate(
            conn,
            None,
            &license_plate
        ).await?;

        if result.len() > 1 {
            return Err(create_new_error!(
                format!("More than one active entry was found for license plate: {}", license_plate)
            ))
        }

        Ok(result.pop().map(Blacklist::from))
    }
    
    pub async fn check_available_to_insert(
        conn: &mut Conn,
        post_request: BlacklistPost
    ) -> TheResult<bool> {
        let result = DbBlacklist::select_by_clients_id_or_license_plate(
            conn,
            post_request.clients_id,
            &post_request.license_plate
        ).await?;
        
        if !result.is_empty() {
            return Ok(false)
        }
        
        Ok(true)
    }
    
    pub async fn create_new_entry(
        conn: &mut Conn,
        post_request: BlacklistPost
    ) -> TheResult<Option<Blacklist>> {
        
        let mut db_blacklist = DbBlacklist::from(post_request);
        
        let result = DbBlacklist::insert_new_entry(
            &mut db_blacklist,
            conn
        ).await?;
        
        if !result {
            return Ok(None)
        }
        
        Ok(Some(Blacklist::from(db_blacklist)))
    }
    
    pub async fn update_blacklist_entry(
        conn: &mut Conn,
        id: BlacklistIdType, 
        patch_request: BlacklistPatch
    ) -> TheResult<bool> {
        DbBlacklist::update_by_id(conn, id, patch_request).await
    }
}
