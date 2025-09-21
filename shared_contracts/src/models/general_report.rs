use chrono::{DateTime, Utc};

use crate::models::money::Money;

pub struct GeneralReport {
    pub trade_period: TradePeriod,
    pub buy_summary: TransactionSummary,
    pub sell_summary: TransactionSummary,
}

pub struct TradePeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

pub struct TransactionSummary {
    pub total_quantity: u32,
    pub total_value: Money,
    pub total_commission: Money,
}
