use std::fs::File;
use std::path::Path;

use chrono::{DateTime, Utc};
use polars::prelude::*;
use shared_contracts::models::report::{Summary, TradePeriod};
use shared_contracts::{
    errors::PortfolioError,
    models::{money::Money, trade_order::OrderSide},
};
use std::io::Write;

pub fn calculate_and_save(input: &Path, output: &Path) -> Result<(), PortfolioError> {
    let df_csv = util::polars::default_lazy_reder(input)?.finish()?;

    let (aggregate_df, summary_df) = create_data_frame(df_csv)?;

    let mut file = File::create(output)?;

    save_metadata(&mut file, summary_df)?;
    save_aggregated_instruments(&mut file, aggregate_df)?;
    Ok(())
}
fn save_aggregated_instruments(
    output: &mut File,
    aggregate_df: LazyFrame,
) -> Result<(), PortfolioError> {
    let aggregate_res = aggregate_df.collect()?;

    let mut selected_col = aggregate_res.select([
        "instrument",
        "net_profit",
        "pct_change",
        "total_commission",
        "tax_amount",
        "buy_quantity",
        "sell_quantity",
        "days_to_settle",
    ])?;

    util::polars::default_writer(output)?.finish(&mut selected_col)?;

    Ok(())
}

fn save_metadata(output: &mut File, summary_df: LazyFrame) -> Result<(), PortfolioError> {
    let summary = map_summary(summary_df)?;

    let metadata = format!(
        "--- Profit Report ---\n
        Trade from {} to {}\n
        Commission: {}\n
        Tax: {}\n
        Net Profit: {}\n\n",
        summary.trade_period.start,
        summary.trade_period.end,
        summary.commission_total.as_string(),
        summary.tax_amount_total.as_string(),
        summary.net_profit_total.as_string()
    );
    writeln!(output, "{}", metadata)?;
    Ok(())
}
fn create_data_frame(dataset: LazyFrame) -> Result<(LazyFrame, LazyFrame), PortfolioError> {
    let round = 2;
    let mode = RoundMode::HalfToEven;

    let df = dataset
        .clone()
        .lazy()
        .filter(
            col("status")
                .eq(lit("Filled"))
                .or(col("status").eq(lit("PartiallyFilled"))),
        )
        .group_by([col("instrument")])
        .agg([
            col("submission_time").min().alias("trade_period_start"),
            col("submission_time").max().alias("trade_period_end"),
            when(col("order_side").eq(lit(OrderSide::Buy.to_string())))
                .then(col("filled_quantity"))
                .otherwise(lit(0_u32))
                .sum()
                .alias("buy_quantity"),
            when(col("order_side").eq(lit(OrderSide::Sell.to_string())))
                .then(col("filled_quantity"))
                .otherwise(lit(0_u32))
                .sum()
                .alias("sell_quantity"),
            when(col("order_side").eq(lit(OrderSide::Buy.to_string())))
                .then(col("commission"))
                .otherwise(lit(0_f64))
                .sum()
                .round(round, mode)
                .alias("buy_commission"),
            when(col("order_side").eq(lit(OrderSide::Sell.to_string())))
                .then(col("commission"))
                .otherwise(lit(0_f64))
                .sum()
                .round(round, mode)
                .alias("sell_commission"),
            when(col("order_side").eq(lit(OrderSide::Buy.to_string())))
                .then(col("price") * col("filled_quantity"))
                .otherwise(lit(0_f64))
                .sum()
                .round(round, mode)
                .alias("purchase_value"),
            when(col("order_side").eq(lit(OrderSide::Sell.to_string())))
                .then(col("price") * col("filled_quantity"))
                .otherwise(lit(0_f64))
                .sum()
                .round(round, mode)
                .alias("sale_value"),
        ])
        // Obliczenia bazowe
        .with_columns([
            (col("buy_commission") + col("sell_commission"))
                .round(round, mode)
                .alias("total_commission"),
            (col("purchase_value") + col("buy_commission")).alias("cost_basis"),
            (col("sale_value") - col("sell_commission")).alias("net_proceeds"),
            (col("trade_period_end") - col("trade_period_start")).alias("settlement_duration"),
        ])
        .with_columns([col("settlement_duration")
            .dt()
            .total_days()
            .alias("days_to_settle")])
        // Średnia cena zakupu
        .with_columns([(col("cost_basis") / col("buy_quantity"))
            .round(round, mode)
            .alias("average_cost_basis")])
        //  Oblicza podatek tylko od sprzedanej ilości, używając średniej ceny
        .with_columns([
            (col("net_proceeds") - (col("average_cost_basis") * col("sell_quantity")))
                .round(round, mode)
                .alias("tax_base"),
        ])
        .with_columns([(col("tax_base") * lit(0.19))
            .round(round, mode)
            .alias("tax_amount")])
        // Zysk netto
        .with_columns([(
            // Jeśli istnieje jakakolwiek sprzedaż
            when(col("sell_quantity").gt(lit(0_u32)))
                // Wtedy oblicz zysk/stratę
                .then(col("net_proceeds") - col("cost_basis") - col("tax_amount"))
                // W przeciwnym razie ustaw 0 (transakcja nierozliczona)
                .otherwise(lit(0_f64))
                .round(round, mode)
                .alias("net_profit")
        )])
        .with_column(
            ((col("tax_base") / col("cost_basis")) * lit(100))
                .round(round, mode)
                .alias("pct_change"),
        )
        .sort(["instrument"], Default::default());

    let summary = df.clone().lazy().select([
        col("trade_period_start").min().alias("trade_period_start"),
        col("trade_period_end").max().alias("trade_period_end"),
        (col("total_commission"))
            .sum()
            .round(round, mode)
            .alias("commission_total"),
        col("tax_amount")
            .sum()
            .round(round, mode)
            .alias("total_tax_amount"),
        col("net_profit")
            .sum()
            .round(round, mode)
            .alias("total_net_profit"),
    ]);

    Ok((df, summary))
}

fn map_summary(summary_df: LazyFrame) -> Result<Summary, PortfolioError> {
    let summary = summary_df.collect()?;
    let res = Summary {
        trade_period: _trade_period(&summary)?,
        commission_total: _money(&summary, "commission_total")?,
        tax_amount_total: _money(&summary, "total_tax_amount")?,
        net_profit_total: _money(&summary, "total_net_profit")?,
    };
    Ok(res)
}

fn _money(df: &DataFrame, column: &str) -> Result<Money, PortfolioError> {
    let val = df
        .column(column)?
        .f64()?
        .get(0)
        .ok_or(PortfolioError::InvalidValue(format!(
            "missing column {column} in summary"
        )))?;
    Ok(Money::from_f64(val))
}

fn _trade_period(df: &DataFrame) -> Result<TradePeriod, PortfolioError> {
    let start = df
        .column("trade_period_start")?
        .datetime()?
        .cast_time_unit(TimeUnit::Nanoseconds)
        .cast(&DataType::Int64)?
        .i64()?
        .get(0)
        .ok_or(PortfolioError::InvalidValue(
            "summary: trade_period_start".into(),
        ))?;

    let end = df
        .column("trade_period_end")?
        .datetime()?
        .cast_time_unit(TimeUnit::Nanoseconds)
        .cast(&DataType::Int64)?
        .i64()?
        .get(0)
        .ok_or(PortfolioError::InvalidValue(
            "summary: trade_period_end".into(),
        ))?;

    Ok(TradePeriod {
        start: DateTime::<Utc>::from_timestamp_nanos(start),
        end: DateTime::<Utc>::from_timestamp_nanos(end),
    })
}
