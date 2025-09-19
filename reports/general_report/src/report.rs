use chrono::{DateTime, Utc};
use polars::prelude::*;
use shared_contracts::errors::ReportError;
use shared_contracts::models::general_report::{GeneralReport, TradePeriod};
use shared_contracts::models::trade_order::*;
use std::vec;

pub fn create(trade_order: Vec<TradeOrder>) -> Result<GeneralReport, ReportError> {
    let orders = _create_dataframe_with_filled_orders(trade_order);

    let summary = orders?
        .lazy()
        .select([
            col("submission_time").min().alias("trade_period_start"),
            col("submission_time").max().alias("trade_period_end"),
        ])
        .collect()?;

    let report = GeneralReport {
        trade_period: TradePeriod {
            start: _to_datetime(summary.column("trade_period_start")?.get(0)?)?,
            end: _to_datetime(summary.column("trade_period_end")?.get(0)?)?,
        },
    };

    Ok(report)
}

fn _to_datetime(val: AnyValue) -> Result<DateTime<Utc>, ReportError> {
    match val {
        AnyValue::Datetime(ns, _, _) => Ok(DateTime::<Utc>::from_timestamp_nanos(ns)),
        _ => Err(ReportError::Error(
            "Failed to convert AnyValue to Datetime".to_string(),
        )),
    }
}

fn _create_dataframe_with_filled_orders(
    trade_orders: Vec<TradeOrder>,
) -> Result<DataFrame, ReportError> {
    let mut submission_time: Vec<i64> = Vec::new();

    for order in trade_orders.iter() {
        if order.status != OrderStatus::Filled {
            continue;
        }
        submission_time.push(order.submission_time.timestamp_nanos_opt().ok_or(
            ReportError::InputError("Can't convert submission_time to timestamp_nanos".to_string()),
        )?);
    }

    let series_submission_time = Series::new("submission_time".into(), submission_time).cast(
        &DataType::Datetime(TimeUnit::Nanoseconds, Some(TimeZone::UTC)),
    )?;

    DataFrame::new(vec![series_submission_time.into()]).map_err(ReportError::CalculationError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use rust_decimal::Decimal;

    fn _orders() -> Vec<TradeOrder> {
        vec![
            TradeOrder {
                instrument_symbol: "AAPL".to_string(),
                instrument_type: InstrumentType::Stock,
                order_type: OrderType::Market,
                side: OrderSide::Buy,
                quantity: 10,
                filled_quantity: 10,
                price: Some(Decimal::new(15000, 2)), // $150.00
                commission: Decimal::new(1, 2),      // $0.01
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
                quantity: 10,
                filled_quantity: 10,
                price: Some(Decimal::new(10, 2)),
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
                filled_quantity: 10,
                price: Some(Decimal::new(20, 2)),
                commission: Decimal::new(1, 2), // $0.01
                status: OrderStatus::Filled,
                submission_time: Utc.with_ymd_and_hms(2025, 2, 15, 10, 0, 0).unwrap(),
                currency: "USD".to_string(),
                exchange: "NASDAQ".to_string(),
            },
        ]
    }

    #[test]
    fn test_trade_period_calculation() {
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
}
