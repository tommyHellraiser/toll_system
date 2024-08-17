

mod test {
    use crate::modules::transaction_logs::{PaymentMethod, TransactionOrigin, TransactionStatus};

    //  PAYMENT METHOD - TO STRING
    #[test]
    fn payment_method_to_string_cash() {
        let value = PaymentMethod::Cash;

        assert_eq!("Cash", &value.to_string())
    }
    
    #[test]
    fn payment_method_to_string_prepaid() {
        let value = PaymentMethod::Prepaid;

        assert_eq!("Prepaid", &value.to_string())
    }
    
    #[test]
    fn payment_method_to_string_credit() {
        let value = PaymentMethod::Credit;

        assert_eq!("Credit", &value.to_string())
    }
    
    #[test]
    fn payment_method_to_string_debit() {
        let value = PaymentMethod::Debit;

        assert_eq!("Debit", &value.to_string())
    }
    
    #[test]
    fn payment_method_to_string_other() {
        let value = PaymentMethod::Other;

        assert_eq!("Other", &value.to_string())
    }

    #[test]
    fn payment_method_to_string_unknown() {
        let value = PaymentMethod::Unknown;

        assert_eq!("Unknown", &value.to_string())
    }

    
    //  PAYMENT METHOD - FROM STRING
    #[test]
    fn payment_method_from_string_cash() {
        let value = String::from("Cash");

        assert_eq!(PaymentMethod::Cash, PaymentMethod::from(value))
    }

    #[test]
    fn payment_method_from_string_prepaid() {
        let value = String::from("Prepaid");

        assert_eq!(PaymentMethod::Prepaid, PaymentMethod::from(value))
    }

    #[test]
    fn payment_method_from_string_credit() {
        let value = String::from("Credit");

        assert_eq!(PaymentMethod::Credit, PaymentMethod::from(value))
    }

    #[test]
    fn payment_method_from_string_debit() {
        let value = String::from("Debit");

        assert_eq!(PaymentMethod::Debit, PaymentMethod::from(value))
    }

    #[test]
    fn payment_method_from_string_other() {
        let value = String::from("Other");

        assert_eq!(PaymentMethod::Other, PaymentMethod::from(value))
    }

    #[test]
    fn payment_method_from_string_unknown() {
        let value = String::from("Unknown");

        assert_eq!(PaymentMethod::Unknown, PaymentMethod::from(value))
    }

    #[test]
    fn payment_method_from_string_unknown_2() {
        let value = String::from("adasd");

        assert_eq!(PaymentMethod::Unknown, PaymentMethod::from(value))
    }

    
    //  TRANSACTION ORIGIN - TO STRING
    #[test]
    fn transaction_origin_to_string_balance_recharge() {
        let value = TransactionOrigin::BalanceRecharge;

        assert_eq!("BalanceRecharge", &value.to_string())
    }

    #[test]
    fn transaction_origin_to_string_transit() {
        let value = TransactionOrigin::Transit;

        assert_eq!("Transit", &value.to_string())
    }

    #[test]
    fn transaction_origin_to_string_penalty() {
        let value = TransactionOrigin::Penalty;

        assert_eq!("Penalty", &value.to_string())
    }

    #[test]
    fn transaction_origin_to_string_undefined() {
        let value = TransactionOrigin::Undefined;

        assert_eq!("Undefined", &value.to_string())
    }

    
    //  TRANSACTION ORIGIN FROM STRING
    #[test]
    fn transaction_origin_from_string_balance_recharge() {
        let value = String::from("BalanceRecharge");

        assert_eq!(TransactionOrigin::BalanceRecharge, TransactionOrigin::from(value))
    }

    #[test]
    fn transaction_origin_from_string_transit() {
        let value = String::from("Transit");

        assert_eq!(TransactionOrigin::Transit, TransactionOrigin::from(value))
    }
    #[test]
    fn transaction_origin_from_string_penalty() {
        let value = String::from("Penalty");

        assert_eq!(TransactionOrigin::Penalty, TransactionOrigin::from(value))
    }
    #[test]
    fn transaction_origin_from_string_undefined() {
        let value = String::from("Undefined");

        assert_eq!(TransactionOrigin::Undefined, TransactionOrigin::from(value))
    }
    #[test]
    fn transaction_origin_from_string_undefined_2() {
        let value = String::from("ajhsvdksjakdf");

        assert_eq!(TransactionOrigin::Undefined, TransactionOrigin::from(value))
    }
    
    //  TRANSACTION STATUS - TO STRING
    #[test]
    fn transaction_status_to_string_confirmed() {
        let value = TransactionStatus::Confirmed;

        assert_eq!("Confirmed", &value.to_string())
    }

    #[test]
    fn transaction_status_to_string_failed() {
        let value = TransactionStatus::Failed;

        assert_eq!("Failed", &value.to_string())
    }

    #[test]
    fn transaction_status_to_string_unknown() {
        let value = TransactionStatus::Unknown;

        assert_eq!("Unknown", &value.to_string())
    }
    
    //  TRANSACTION STATUS - FROM STRING
    #[test]
    fn transaction_status_from_string_confirmed() {
        let value = String::from("Confirmed");

        assert_eq!(TransactionStatus::Confirmed, TransactionStatus::from(value))
    }

    #[test]
    fn transaction_status_from_string_failed() {
        let value = String::from("Failed");

        assert_eq!(TransactionStatus::Failed, TransactionStatus::from(value))
    }

    #[test]
    fn transaction_status_from_string_unknown() {
        let value = String::from("Unknown");

        assert_eq!(TransactionStatus::Unknown, TransactionStatus::from(value))
    }

    #[test]
    fn transaction_status_from_string_unknown_2() {
        let value = String::from("ljkadsebhlkadjsfg");

        assert_eq!(TransactionStatus::Unknown, TransactionStatus::from(value))
    }
}