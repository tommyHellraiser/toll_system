use rust_decimal::Decimal;
use crate::utilities::datatypes::{ClientsBalancesIdType, ClientsIdType};

mod db;

struct ClientsBalances {
    id: ClientsBalancesIdType,
    clients_id: ClientsIdType,
    balance: Decimal,
}
