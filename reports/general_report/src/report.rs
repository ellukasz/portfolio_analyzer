use crate::data_frame_factory;
use chrono::{DateTime, Utc};
use polars::prelude::*;
use rust_decimal::Decimal;
use shared_contracts::errors::ReportError;
use shared_contracts::models::general_report::*;
use shared_contracts::models::trade_order::{DEFAULT_MONEY_SCALE, OrderSide, TradeOrder};

pub fn create(trade_order: Vec<TradeOrder>) -> Result<GeneralReport, ReportError> {
    let orders = data_frame_factory::create_with_filled_orders(trade_order)?;

    let summary = orders
        .clone()
        .lazy()
        .select([
            col("submission_time").min().alias("trade_period_start"),
            col("submission_time").max().alias("trade_period_end"),
            when(col("order_side").eq(lit(OrderSide::Buy.to_string())))
                .then(col("filled_quantity"))
                .otherwise(lit(0_u32))
                .sum()
                .alias("buy_total_quantity"),
            when(col("order_side").eq(lit(OrderSide::Buy.to_string())))
                .then(col("price") * col("filled_quantity"))
                .otherwise(lit(0_i128))
                .sum()
                .alias("buy_price"),
        ])
        .collect()?;

    let report = GeneralReport {
        trade_period: TradePeriod {
            start: _to_datetime(summary.column("trade_period_start")?.get(0)?)?,
            end: _to_datetime(summary.column("trade_period_end")?.get(0)?)?,
        },
        buy_summary: TransactionSummary {
            total_quantity: _to_u32(summary.column("buy_total_quantity")?.get(0)?)?,
            total_value: _to_decimal(summary.column("buy_price")?.get(0)?)?,
        },
    };

    Ok(report)
}
fn _to_decimal(val: AnyValue) -> Result<Decimal, ReportError> {
    match val {
        AnyValue::Int128(v) => Ok(Decimal::from_i128_with_scale(v, DEFAULT_MONEY_SCALE)),
        _ => Err(ReportError::Error(
            "Failed to convert AnyValue to Decimal".to_string(),
        )),
    }
}
fn _to_u32(val: AnyValue) -> Result<u32, ReportError> {
    match val {
        AnyValue::UInt32(v) => Ok(v),
        AnyValue::Int32(v) if v >= 0 => Ok(v as u32),
        _ => Err(ReportError::Error(
            "Failed to convert AnyValue to u32".to_string(),
        )),
    }
}
fn _to_datetime(val: AnyValue) -> Result<DateTime<Utc>, ReportError> {
    match val {
        AnyValue::Datetime(ns, _, _) => Ok(DateTime::<Utc>::from_timestamp_nanos(ns)),
        _ => Err(ReportError::Error(
            "Failed to convert AnyValue to Datetime".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use rust_decimal::Decimal;
    use shared_contracts::models::trade_order::*;

    fn _orders() -> Vec<TradeOrder> {
        vec![
            TradeOrder {
                instrument_symbol: "AAPL".to_string(),
                instrument_type: InstrumentType::Stock,
                order_type: OrderType::Market,
                side: OrderSide::Buy,
                quantity: 10,
                filled_quantity: 1,
                price: Some(Decimal::from_i128_with_scale(1000, DEFAULT_MONEY_SCALE)), // $100.00
                commission: Decimal::new(1, 2),                                        // $0.01
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
                price: Some(Decimal::from_i128_with_scale(1000, DEFAULT_MONEY_SCALE)),
                commission: Decimal::new(1, 2), // $0.01
                status: OrderStatus::Filled,
                submission_time: Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap(),
                currency: "USD".to_string(),
                exchange: "NASDAQ".to_string(),
            },
            TradeOrder {
                instrument_symbol: "AAPL".to_string(),
                instrument_type: InstrumentType::Stock,
                order_type: OrderType::Market,
                side: OrderSide::Sell,
                quantity: 20,
                filled_quantity: 2,
                price: Some(Decimal::from_i128_with_scale(2000, DEFAULT_MONEY_SCALE)),
                commission: Decimal::from_i128_with_scale(2, DEFAULT_MONEY_SCALE),
                status: OrderStatus::Filled,
                submission_time: Utc.with_ymd_and_hms(2025, 2, 15, 10, 0, 0).unwrap(),
                currency: "USD".to_string(),
                exchange: "NASDAQ".to_string(),
            },
        ]
    }

    #[test]
    fn trade_period_calculation() {
        let report = create(_orders()).unwrap();
        assert_eq!(
            report.trade_period.start,
            Utc.with_ymd_and_hms(2023, 1, 15, 10, 0, 0).unwrap()
        );
        assert_eq!(
            report.trade_period.end,
            Utc.with_ymd_and_hms(2025, 2, 15, 10, 0, 0).unwrap()
        );
    }

    #[test]
    fn buy_total_quantity() {
        let report = create(_orders()).unwrap();
        assert_eq!(report.buy_summary.total_quantity, 20);
    }
    #[test]
    fn buy_total_value() {
        let report = create(_orders()).unwrap();
        assert_eq!(
            report.buy_summary.total_value,
            Decimal::new(3000, DEFAULT_MONEY_SCALE)
        );
    }
}
