use rust_decimal::Decimal;
use crate::modules::transaction_logs::PaymentMethod;
use crate::utilities::datatypes::{DiscountsIdType, TransitFeeLogsIdType, TransitRatesIdType};

mod db;

struct TransitFeeLogs {
    id: TransitFeeLogsIdType,
    original_amount: Decimal,
    has_client_discount: bool,
    transit_rates_id: TransitRatesIdType,
    discounts_id: DiscountsIdType,
    final_amount: Decimal,
    payment_method: PaymentMethod,
}
