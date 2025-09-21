use chrono::TimeZone;
use chrono::Utc;
use shared_contracts::models::money::Money;
use shared_contracts::models::trade_order::*;

pub fn orders() -> Vec<TradeOrder> {
    vec![
        //buy orders
        TradeOrder {
            instrument_symbol: "AAPL".to_string(),
            instrument_type: InstrumentType::Stock,
            order_type: OrderType::Market,
            side: OrderSide::Buy,
            quantity: 10,
            filled_quantity: 1,
            price: Some(Money::from_i128(1000)), // $100.00
            commission: Money::from_i128(100),   // $1
            status: OrderStatus::Filled,
            submission_time: Utc.with_ymd_and_hms(2023, 1, 15, 10, 0, 0).unwrap(),
            currency: "USD".to_string(),
            exchange: "NASDAQ".to_string(),
        },
        TradeOrder {
            instrument_symbol: "AAPL".to_string(),
            instrument_type: InstrumentType::Stock,
            order_type: OrderType::Market,
            side: OrderSide::Buy,
            quantity: 2,
            filled_quantity: 2,
            price: Some(Money::from_i128(1000)),
            commission: Money::from_i128(200),
            status: OrderStatus::Filled,
            submission_time: Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap(),
            currency: "USD".to_string(),
            exchange: "NASDAQ".to_string(),
        },
        //sell orders
        TradeOrder {
            instrument_symbol: "AAPL".to_string(),
            instrument_type: InstrumentType::Stock,
            order_type: OrderType::Market,
            side: OrderSide::Sell,
            quantity: 10,
            filled_quantity: 1,
            price: Some(Money::from_i128(1000)), // $100.00
            commission: Money::from_i128(100),   // $1
            status: OrderStatus::Filled,
            submission_time: Utc.with_ymd_and_hms(2025, 2, 15, 10, 0, 0).unwrap(),
            currency: "USD".to_string(),
            exchange: "NASDAQ".to_string(),
        },
        TradeOrder {
            instrument_symbol: "AAPL".to_string(),
            instrument_type: InstrumentType::Stock,
            order_type: OrderType::Market,
            side: OrderSide::Sell,
            quantity: 2,
            filled_quantity: 2,
            price: Some(Money::from_i128(1000)),
            commission: Money::from_i128(200),
            status: OrderStatus::Filled,
            submission_time: Utc.with_ymd_and_hms(2025, 2, 15, 10, 0, 0).unwrap(),
            currency: "USD".to_string(),
            exchange: "NASDAQ".to_string(),
        },
    ]
}
