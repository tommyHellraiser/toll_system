use chrono::NaiveDateTime;
use mysql_async::prelude::FromRow;
use mysql_async::{FromRowError, Row};
use rust_decimal::Decimal;
use crate::get_value_from_row;
use crate::modules::transaction_logs::{PaymentMethod, TransactionOrigin, TransactionStatus};
use crate::utilities::datatypes::{ClientsIdType, EventIdType, ExternalReferenceIdType, TransactionLogsIdType};

pub struct DbTransactionLogs {
    id: TransactionLogsIdType,
    clients_id: Option<ClientsIdType>,
    amount: Decimal,
    payment_method: PaymentMethod,
    origin: Option<TransactionOrigin>,
    status: TransactionStatus,
    event_id: Option<EventIdType>,
    transaction_time: NaiveDateTime,
    external_reference_id: Option<ExternalReferenceIdType>,
    error: Option<String>
}

impl FromRow for DbTransactionLogs {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let table = "transaction_logs";
        Self {
            id: get_value_from_row!(row, "ID", table, TransactionLogsIdType),
            clients_id: get_value_from_row!(row, "clients_ID", table, Option<ClientsIdType>),
            amount: get_value_from_row!(row, "amount", table, Decimal),
            payment_method: get_value_from_row!(row, "payment_method", table, String).into(),
            origin: get_value_from_row!(row, "origin", table, Option<String>).map(|origin| origin.into()),
            status: get_value_from_row!(row, "status", table, String).into(),
            event_id: get_value_from_row!(row, "event_ID", table, Option<EventIdType>),
            transaction_time: get_value_from_row!(row, "transaction_time", table, NaiveDateTime),
            external_reference_id: get_value_from_row!(row, "external_reference_id", table, Option<ExternalReferenceIdType>),
            error: get_value_from_row!(row, "error", table, Option<String>)
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
