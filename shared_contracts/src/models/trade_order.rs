use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TradeOrder {
    /// Symbol or identifier of a financial instrument (e.g. "AAPL", "EURUSD", "PLN=F").
    pub instrument_symbol: String,

    /// Type of financial instrument (e.g. Stock, CurrencyPair, Future, Bond).
    pub instrument_type: InstrumentType,

    /// Type of order (e.g. Market, Limit, Stop).
    pub order_type: OrderType,

    /// Side of the order (Buy or Sell).
    pub side: OrderSide,

    /// Quantity that was ordered.
    pub quantity: u32,

    /// Quantity of the instrument that has been filled/executed.
    pub filled_quantity: u32,

    /// Price of the order. Optional, as Market orders do not have a specified price.
    pub price: Option<Decimal>,

    /// Commission charged for the order.
    pub commission: Decimal,

    /// Current status of the order.
    pub status: OrderStatus,

    /// The exact time of placing the order.
    pub submission_time: DateTime<Utc>,

    /// Currency in which the order is settled (e.g. "USD", "PLN", "EUR").
    pub currency: String,

    /// Name or identifier of the exchange/trading platform (e.g. "NASDAQ", "GPW", "FXCM").
    pub exchange: String,
}

/// Types of financial instruments.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InstrumentType {
    Stock,
}

/// Stock order types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrderType {
    Market,
    Limit,
    StopLimit,
}

/// Side of the order (Buy or Sell).

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Status of the order.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrderStatus {
    Pending,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
    Expired,
}
