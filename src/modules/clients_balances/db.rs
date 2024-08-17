use mysql_async::prelude::FromRow;
use mysql_async::{FromRowError, Row};
use rust_decimal::Decimal;
use crate::get_value_from_row;
use crate::modules::clients_balances::ClientsBalances;
use crate::utilities::datatypes::{ClientsBalancesIdType, ClientsIdType};

struct DbClientsBalances {
    id: ClientsBalancesIdType,
    clients_id: ClientsIdType,
    balance: Decimal,
}

impl FromRow for DbClientsBalances {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let table = "clients_balances";
        Self {
            id: get_value_from_row!(row, "ID", table, ClientsBalancesIdType),
            clients_id: get_value_from_row!(row, "clients_ID", table, ClientsIdType),
            balance: get_value_from_row!(row, "balance", table, Decimal)
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl From<DbClientsBalances> for ClientsBalances {
    fn from(value: DbClientsBalances) -> Self {
        Self {
            id: value.id,
            clients_id: value.clients_id,
            balance: value.balance
        }
    }
}
