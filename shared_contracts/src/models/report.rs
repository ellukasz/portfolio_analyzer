use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::money::Money;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfitReport {
    #[serde(flatten)]
    pub summary: Summary,
    #[serde(flatten)]
    pub instruments: Vec<Instrument>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    pub trade_period: TradePeriod,
    pub commission_total: Money,
    pub tax_amount_total: Money,
    pub net_profit_total: Money,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Instrument {
    pub instrument_symbol: String,
    pub trade_period: TradePeriod,
    pub buy_quantity: u32,
    pub sell_quantity: u32,
    pub buy_commission: Money,
    pub sell_commission: Money,
    pub total_commission: Money,
    pub purchase_value: Money,
    pub sale_value: Money,
    pub cost_basis: Money,
    pub net_proceeds: Money,
    pub average_cost_basis: Money,
    pub tax_amount: Money,
    pub net_profit: Money,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradePeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}
