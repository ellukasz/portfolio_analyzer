use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

pub struct GeneralReport {
    pub trade_period: TradePeriod,
    pub buy_summary: TransactionSummary,
}

pub struct TradePeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

pub struct TransactionSummary {
    pub total_quantity: u32,
    pub total_value: Decimal,
}
