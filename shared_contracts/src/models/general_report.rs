use chrono::{DateTime, Utc};

use crate::models::money::Money;

pub struct GeneralReport {
    pub trade_period: TradePeriod,
    pub buy_summary: TransactionSummary,
    pub sell_summary: TransactionSummary,
    pub profit_summary: ProfitSummary,
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

pub struct ProfitSummary {
    pub gross_profit: Money,
    pub gross_price_with_commission: Money,
    pub tax: Money,
    pub net_profit: Money,
}
