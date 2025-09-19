use chrono::{DateTime, Utc};

pub struct GeneralReport {
    pub trade_period: TradePeriod,
}

pub struct TradePeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}
