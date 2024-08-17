use std::fmt::{Display, Formatter};
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use crate::utilities::datatypes::{ClientsIdType, EventIdType, ExternalReferenceIdType, TransactionLogsIdType};

mod db;
#[cfg(test)]
mod tests;

pub(super) struct TransactionLogs {
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

#[derive(PartialOrd, PartialEq, Debug)]
pub(super) enum PaymentMethod {
    Cash,
    Prepaid,
    Credit,
    Debit,
    Other,
    Unknown
}

#[derive(PartialOrd, PartialEq, Debug)]
pub(super) enum TransactionOrigin {
    BalanceRecharge,
    Transit,
    Penalty,
    Undefined
}

#[derive(PartialOrd, PartialEq, Debug)]
pub(super) enum TransactionStatus {
    Confirmed,
    Failed,
    Unknown
}

impl Display for PaymentMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            PaymentMethod::Cash => { "Cash".to_string() },
            PaymentMethod::Prepaid => { "Prepaid".to_string() },
            PaymentMethod::Credit => { "Credit".to_string() },
            PaymentMethod::Debit => { "Debit".to_string() },
            PaymentMethod::Other => { "Other".to_string() },
            PaymentMethod::Unknown => { "Unknown".to_string() }
        };
        write!(f, "{}", content)
    }
}

impl From<String> for PaymentMethod {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Cash" => { PaymentMethod::Cash },
            "Prepaid" => { PaymentMethod::Prepaid },
            "Credit" => { PaymentMethod::Credit },
            "Debit" => { PaymentMethod::Debit },
            "Other" => { PaymentMethod::Other },
            _ => { PaymentMethod::Unknown }
        }
    }
}

impl Display for TransactionOrigin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            TransactionOrigin::BalanceRecharge => { "BalanceRecharge".to_string() },
            TransactionOrigin::Transit => { "Transit".to_string() },
            TransactionOrigin::Penalty => { "Penalty".to_string() },
            TransactionOrigin::Undefined => { "Undefined".to_string() }
        };
        write!(f, "{}", content)
    }
}

impl From<String> for TransactionOrigin {
    fn from(value: String) -> Self {
        match value.as_str() {
            "BalanceRecharge" => { TransactionOrigin::BalanceRecharge },
            "Transit" => { TransactionOrigin::Transit },
            "Penalty" => { TransactionOrigin::Penalty }
            _ => { TransactionOrigin::Undefined }
        }
    }
}

impl Display for TransactionStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            TransactionStatus::Confirmed => { "Confirmed".to_string() },
            TransactionStatus::Failed => { "Failed".to_string() },
            TransactionStatus::Unknown => { "Unknown".to_string() },
        };
        write!(f, "{}", content)
    }
}

impl From<String> for TransactionStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Confirmed" => { TransactionStatus::Confirmed },
            "Failed" => { TransactionStatus::Failed },
            "Unknown" => { TransactionStatus::Unknown }
            _ => { TransactionStatus::Unknown }
        }
    }
}
