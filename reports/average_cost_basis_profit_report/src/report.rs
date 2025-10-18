use std::path::Path;

use crate::data_frame_factory;
use chrono::{DateTime, Utc};
use itertools::izip;
use polars::prelude::*;
use shared_contracts::models::report::{Instrument, ProfitReport, Summary, TradePeriod};
use shared_contracts::{
    errors::ReportError,
    models::{
        money::Money,
        trade_order::{OrderSide, TradeOrder},
    },
};

pub fn from_csv(trade_order_csv: &Path) -> Result<ProfitReport, ReportError> {
    let df_csv = CsvReadOptions::default()
        .with_has_header(true)
        .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
        .try_into_reader_with_file_path(Some(trade_order_csv.to_path_buf()))?
        .finish()?;

    create(df_csv)
}

pub fn from_vec(trade_order: Vec<TradeOrder>) -> Result<ProfitReport, ReportError> {
    let dataset = data_frame_factory::create_with_filled_orders(trade_order)?;
    create(dataset)
}

fn create(dataset: DataFrame) -> Result<ProfitReport, ReportError> {
    let df: DataFrame = dataset
        .clone()
        .lazy()
        .group_by([col("instrument_symbol")])
        .agg([
            col("instrument_symbol").first().alias("ticker"),
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
                .otherwise(lit(0_i128))
                .sum()
                .alias("buy_commission"),
            when(col("order_side").eq(lit(OrderSide::Sell.to_string())))
                .then(col("commission"))
                .otherwise(lit(0_i128))
                .sum()
                .alias("sell_commission"),
            when(col("order_side").eq(lit(OrderSide::Buy.to_string())))
                .then(col("price") * col("filled_quantity"))
                .otherwise(lit(0_i128))
                .sum()
                .alias("purchase_value"),
            when(col("order_side").eq(lit(OrderSide::Sell.to_string())))
                .then(col("price") * col("filled_quantity"))
                .otherwise(lit(0_i128))
                .sum()
                .alias("sale_value"),
        ])
        // Obliczenia bazowe
        .with_columns([
            (col("buy_commission") + col("sell_commission")).alias("total_commission"),
            (col("purchase_value") + col("buy_commission")).alias("cost_basis"),
            (col("sale_value") - col("sell_commission")).alias("net_proceeds"),
        ])
        // Średnia cena zakupu
        .with_columns([(col("cost_basis") / col("buy_quantity")).alias("average_cost_basis")])
        //  Oblicza podatek tylko od sprzedanej ilości, używając średniej ceny
        .with_columns([
            (col("net_proceeds") - (col("average_cost_basis") * col("sell_quantity")))
                .alias("tax_base"),
        ])
        .with_columns([((col("tax_base") * lit(19_i128)) / lit(100_i128)).alias("tax_amount")])
        // Zysk netto
        .with_columns([(
            // Jeśli istnieje jakakolwiek sprzedaż
            when(col("sell_quantity").gt(lit(0_i128)))
                .then(
                    // Wtedy oblicz zysk/stratę
                    col("net_proceeds") - col("cost_basis") - col("tax_amount"),
                )
                .otherwise(
                    // W przeciwnym razie ustaw 0 (transakcja nierozliczona)
                    lit(0_i128),
                )
                .alias("net_profit")
        )])
        .sort(["instrument_symbol"], Default::default())
        .collect()?;

    let summary = df
        .clone()
        .lazy()
        .select([
            col("trade_period_start").min().alias("trade_period_start"),
            col("trade_period_end").max().alias("trade_period_end"),
            (col("total_commission")).sum().alias("commission_total"),
            col("tax_amount").sum().alias("total_tax_amount"),
            col("net_profit").sum().alias("total_net_profit"),
        ])
        .collect()?;

    let report = ProfitReport {
        summary: Summary {
            trade_period: _trade_period(&summary)?,
            commission_total: _money(&summary, "commission_total")?,
            tax_amount_total: _money(&summary, "total_tax_amount")?,
            net_profit_total: _money(&summary, "total_net_profit")?,
        },
        instruments: _crete_instrument_report(&df)?,
    };

    Ok(report)
}

fn _money(df: &DataFrame, column: &str) -> Result<Money, ReportError> {
    let val = df
        .column(column)?
        .i128()?
        .get(0)
        .ok_or(ReportError::MissingData(format!(
            "missing column {column} in summary"
        )))?;

    Ok(Money::from_i128(val))
}

fn _trade_period(df: &DataFrame) -> Result<TradePeriod, ReportError> {
    let start = df
        .column("trade_period_start")?
        .i64()?
        .get(0)
        .ok_or(ReportError::MissingData(
            "summary: trade_period_start".into(),
        ))?;

    let end = df
        .column("trade_period_end")?
        .i64()?
        .get(0)
        .ok_or(ReportError::MissingData("summary: trade_period_end".into()))?;

    Ok(TradePeriod {
        start: DateTime::<Utc>::from_timestamp_nanos(start),
        end: DateTime::<Utc>::from_timestamp_nanos(end),
    })
}
struct AggregatedTradeItem<'a> {
    pub ticker: Option<&'a str>,
    pub trade_period_start: Option<i64>,
    pub trade_period_end: Option<i64>,
    pub buy_quantity: Option<u32>,
    pub sell_quantity: Option<u32>,
    pub buy_commission: Option<i128>,
    pub total_commission: Option<i128>,
    pub sell_commission: Option<i128>,
    pub purchase_value: Option<i128>,
    pub sale_value: Option<i128>,
    pub cost_basis: Option<i128>,
    pub net_proceeds: Option<i128>,
    pub average_cost_basis: Option<i128>,
    pub tax_amount: Option<i128>,
    pub net_profit: Option<i128>,
}

fn _crete_instrument_report(df: &DataFrame) -> Result<Vec<Instrument>, ReportError> {
    //         println!("{}", df);

    let combined_itr = _create_combined_trade_iterator(df)?;

    let res: Result<Vec<Instrument>, ReportError> = combined_itr
        .map(|item| {
            let ticker = item
                .ticker
                .ok_or(ReportError::MissingData("ticker".into()))?
                .to_string();

            let start = item
                .trade_period_start
                .ok_or(ReportError::MissingData("trade_period_start".into()))?;

            let end = item
                .trade_period_end
                .ok_or(ReportError::MissingData("trade_period_end".into()))?;

            let buy_quantity = item
                .buy_quantity
                .ok_or(ReportError::MissingData("buy_quantity".into()))?;

            let sell_quantity = item
                .sell_quantity
                .ok_or(ReportError::MissingData("sell_quantity".into()))?;

            let buy_commission = item
                .buy_commission
                .map(Money::from_i128)
                .ok_or(ReportError::MissingData("buy_commission".into()))?;

            let sell_commission = item
                .sell_commission
                .map(Money::from_i128)
                .ok_or(ReportError::MissingData("sell_commission".into()))?;

            let total_commission = item
                .total_commission
                .map(Money::from_i128)
                .ok_or(ReportError::MissingData("total_commission".into()))?;

            let purchase_value = item
                .purchase_value
                .map(Money::from_i128)
                .ok_or(ReportError::MissingData("purchase_value".into()))?;

            let sale_value = item
                .sale_value
                .map(Money::from_i128)
                .ok_or(ReportError::MissingData("sale_value".into()))?;

            let cost_basis = item
                .cost_basis
                .map(Money::from_i128)
                .ok_or(ReportError::MissingData("cost_basis".into()))?;

            let net_proceeds = item
                .net_proceeds
                .map(Money::from_i128)
                .ok_or(ReportError::MissingData("net_proceeds".into()))?;

            let average_cost_basis = item
                .average_cost_basis
                .map(Money::from_i128)
                .ok_or(ReportError::MissingData("average_cost_basis".into()))?;

            let tax_amount = item
                .tax_amount
                .map(Money::from_i128)
                .ok_or(ReportError::MissingData("tax_amount".into()))?;

            let net_profit = item
                .net_profit
                .map(Money::from_i128)
                .ok_or(ReportError::MissingData("net_profit".into()))?;

            Ok(Instrument {
                instrument_symbol: ticker,
                trade_period: TradePeriod {
                    start: DateTime::<Utc>::from_timestamp_nanos(start),
                    end: DateTime::<Utc>::from_timestamp_nanos(end),
                },
                buy_quantity,
                sell_quantity,
                buy_commission,
                sell_commission,
                total_commission,
                purchase_value,
                sale_value,
                cost_basis,
                net_proceeds,
                average_cost_basis,
                tax_amount,
                net_profit,
            })
        })
        .collect();
    res
}

macro_rules! get_i128_iter {
    ($df:expr, $name:expr ) => {
        $df.column($name)?.i128()?.into_iter()
    };
}

macro_rules! get_u32_iter {
    ($df:expr, $name:expr ) => {
        $df.column($name)?.u32()?.into_iter()
    };
}

macro_rules! get_i64_iter {
    ($df:expr, $name:expr ) => {
        $df.column($name)?.i64()?.into_iter()
    };
}
macro_rules! get_str_iter {
    ($df:expr, $name:expr ) => {
        $df.column($name)?.str()?.into_iter()
    };
}

fn _create_combined_trade_iterator<'a>(
    df: &'a DataFrame,
) -> Result<impl Iterator<Item = AggregatedTradeItem<'a>> + 'a, ReportError> {
    let ticker_iter = get_str_iter!(df, "ticker");
    let start_iter = get_i64_iter!(df, "trade_period_start");
    let end_iter = get_i64_iter!(df, "trade_period_end");

    let buy_quantity_iter = get_u32_iter!(df, "buy_quantity");
    let sell_quantity_iter = get_u32_iter!(df, "sell_quantity");

    let buy_commission_iter = get_i128_iter!(df, "buy_commission");
    let sell_commission_iter = get_i128_iter!(df, "sell_commission");
    let total_commission_iter = get_i128_iter!(df, "total_commission");

    let purchase_value_iter = get_i128_iter!(df, "purchase_value");
    let sale_value_iter = get_i128_iter!(df, "sale_value");

    let cost_basis_iter = get_i128_iter!(df, "cost_basis");
    let net_proceeds_iter = get_i128_iter!(df, "net_proceeds");

    let average_cost_basis_iter = get_i128_iter!(df, "average_cost_basis");

    let tax_amount_iter = get_i128_iter!(df, "tax_amount");
    let net_profit_iter = get_i128_iter!(df, "net_profit");

    let combined_itr = izip!(
        ticker_iter,
        start_iter,
        end_iter,
        buy_quantity_iter,
        sell_quantity_iter,
        buy_commission_iter,
        sell_commission_iter,
        total_commission_iter,
        purchase_value_iter,
        sale_value_iter,
        cost_basis_iter,
        net_proceeds_iter,
        average_cost_basis_iter,
        tax_amount_iter,
        net_profit_iter
    )
    .map(
        |(
            ticker,
            trade_period_start,
            trade_period_end,
            buy_quantity,
            sell_quantity,
            buy_commission,
            sell_commission,
            total_commission,
            purchase_value,
            sale_value,
            cost_basis,
            net_proceeds,
            average_cost_basis,
            tax_amount,
            net_profit,
        )| {
            AggregatedTradeItem {
                ticker,
                trade_period_start,
                trade_period_end,
                buy_quantity,
                sell_quantity,
                buy_commission,
                sell_commission,
                total_commission,
                purchase_value,
                sale_value,
                cost_basis,
                net_proceeds,
                average_cost_basis,
                tax_amount,
                net_profit,
            }
        },
    );
    Ok(combined_itr)
}
