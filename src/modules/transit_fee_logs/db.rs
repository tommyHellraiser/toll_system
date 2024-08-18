use mysql_async::prelude::FromRow;
use mysql_async::{FromRowError, Row};
use rust_decimal::Decimal;
use crate::get_value_from_row;
use crate::modules::transaction_logs::PaymentMethod;
use crate::modules::transit_fee_logs::TransitFeeLogs;
use crate::utilities::datatypes::{DiscountsIdType, TransitFeeLogsIdType, TransitRatesIdType};

struct DbTransitFeeLogs {
    id: TransitFeeLogsIdType,
    original_amount: Decimal,
    has_client_discount: bool,
    transit_rates_id: TransitRatesIdType,
    discounts_id: DiscountsIdType,
    final_amount: Decimal,
    payment_method: PaymentMethod,
}

impl FromRow for DbTransitFeeLogs {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let table = "transit_fee_logs";
        Self {
            id: get_value_from_row!(row, "ID", table, TransitFeeLogsIdType),
            original_amount: get_value_from_row!(row, "original_amount", table, Decimal),
            has_client_discount: get_value_from_row!(row, "has_client_discount", table, bool),
            transit_rates_id: get_value_from_row!(row, "transit_rates_ID", table, TransitRatesIdType),
            discounts_id: get_value_from_row!(row, "discounts_ID", table, DiscountsIdType),
            final_amount: get_value_from_row!(row, "final_amount", table, Decimal),
            payment_method: get_value_from_row!(row, "payment_method", table, String).into()
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl From<DbTransitFeeLogs> for TransitFeeLogs {
    fn from(value: DbTransitFeeLogs) -> Self {
        Self {
            id: value.id,
            original_amount: value.original_amount,
            has_client_discount: value.has_client_discount,
            transit_rates_id: value.transit_rates_id,
            discounts_id: value.discounts_id,
            final_amount: value.final_amount,
            payment_method: value.payment_method
        }
    }
}
