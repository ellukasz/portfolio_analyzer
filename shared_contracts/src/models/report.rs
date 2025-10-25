use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    pub commission_total: f64,
    pub tax_amount_total: f64,
    pub net_profit_total: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Instrument {
    pub instrument_symbol: String,
    pub trade_period: TradePeriod,
    pub buy_quantity: i64,
    pub sell_quantity: i64,
    pub buy_commission: f64,
    pub sell_commission: f64,
    pub total_commission: f64,
    pub purchase_value: f64,
    pub sale_value: f64,
    pub cost_basis: f64,
    pub net_proceeds: f64,
    pub average_cost_basis: f64,
    pub tax_amount: f64,
    pub net_profit: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradePeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}
