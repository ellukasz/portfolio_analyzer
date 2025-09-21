use crate::data_frame_factory;
use chrono::{DateTime, Utc};
use polars::prelude::*;
use shared_contracts::errors::ReportError;
use shared_contracts::models::general_report::*;
use shared_contracts::models::money::Money;
use shared_contracts::models::trade_order::{OrderSide, TradeOrder};

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
            when(col("order_side").eq(lit(OrderSide::Buy.to_string())))
                .then(col("commission"))
                .otherwise(lit(0_i128))
                .sum()
                .alias("buy_total_commission"),
        ])
        .collect()?;

    let report = GeneralReport {
        trade_period: TradePeriod {
            start: _to_datetime(summary.column("trade_period_start")?.get(0)?)?,
            end: _to_datetime(summary.column("trade_period_end")?.get(0)?)?,
        },
        buy_summary: TransactionSummary {
            total_quantity: _to_u32(summary.column("buy_total_quantity")?.get(0)?)?,
            total_value: _to_money(summary.column("buy_price")?.get(0)?)?,
            total_commission: _to_money(summary.column("buy_total_commission")?.get(0)?)?,
        },
    };

    Ok(report)
}
fn _to_money(val: AnyValue) -> Result<Money, ReportError> {
    match val {
        AnyValue::Int128(v) => Ok(Money::from_i128(v)),
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
