use chrono::{NaiveDateTime};
use mysql_async::prelude::FromRow;
use mysql_async::{FromRowError, Row};
use crate::get_value_from_row;
use crate::modules::blacklist::Blacklist;
use crate::utilities::datatypes::{BlacklistIdType, ClientsIdType};

#[derive(Default)]
struct DbBlacklist {
    id: BlacklistIdType,
    clients_id: ClientsIdType,
    reason: String,
    restriction_expiry: Option<NaiveDateTime>
}

impl FromRow for DbBlacklist {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let table = "blacklist";
        Self {
            id: get_value_from_row!(row, "ID", table, BlacklistIdType),
            clients_id: get_value_from_row!(row, "clients_ID", table, ClientsIdType),
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
            reason: value.reason,
            restriction_expiry: value.restriction_expiry
        }
    }
}
