use crate::data_frame_factory;
use chrono::{DateTime, Utc};
use itertools::izip;
use polars::prelude::*;
use shared_contracts::{
    errors::ReportError,
    models::{
        money::Money,
        trade_order::{OrderSide, TradeOrder},
    },
};

pub struct Report {
    pub instruments: Vec<Instrument>,
}

pub struct Instrument {
    pub instrument_symbol: String,
    pub trade_period: TradePeriod,
    pub buy_quantity: u32,
    pub sell_quantity: u32,
    pub buy_commission: i128,
    pub sell_commission: i128,
    pub purchase_value: i128,
    pub sale_value: i128,
    pub cost_basis: i128,
    pub net_proceeds: i128,
    pub average_cost_basis: i128,
    pub tax_base: i128,
    pub tax_amount: Money,
}

pub struct TradePeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

pub fn create(trade_order: Vec<TradeOrder>) -> Result<Report, ReportError> {
    let dataset = data_frame_factory::create_with_filled_orders(trade_order)?;

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
        .with_columns([
            (col("purchase_value") + col("buy_commission")).alias("cost_basis"),
            (col("sale_value") - col("sell_commission")).alias("net_proceeds"),
        ])
        .with_columns([(col("cost_basis") / col("buy_quantity")).alias("average_cost_basis")])
        .with_columns([
            (col("net_proceeds") - (col("average_cost_basis") * col("sell_quantity")))
                .alias("tax_base"),
        ])
        .with_columns([((col("tax_base") * lit(19_i128)) / lit(100_i128)).alias("tax_amount")])
        .sort(["instrument_symbol"], Default::default())
        .collect()?;

    let report = Report {
        instruments: _crete_instrument_report(&df)?,
    };

    Ok(report)
}

struct AggregatedTradeItem<'a> {
    pub ticker: Option<&'a str>,
    pub trade_period_start: Option<i64>,
    pub trade_period_end: Option<i64>,
    pub buy_quantity: Option<u32>,
    pub sell_quantity: Option<u32>,
    pub buy_commission: Option<i128>,
    pub sell_commission: Option<i128>,
    pub purchase_value: Option<i128>,
    pub sale_value: Option<i128>,
    pub cost_basis: Option<i128>,
    pub net_proceeds: Option<i128>,
    pub average_cost_basis: Option<i128>,
    pub tax_base: Option<i128>,
    pub tax_amount: Option<i128>,
}

fn _crete_instrument_report(df: &DataFrame) -> Result<Vec<Instrument>, ReportError> {
    //   println!("{}", df);

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
                .ok_or(ReportError::MissingData("buy_commission".into()))?;

            let sell_commission = item
                .sell_commission
                .ok_or(ReportError::MissingData("sell_commission".into()))?;

            let purchase_value = item
                .purchase_value
                .ok_or(ReportError::MissingData("purchase_value".into()))?;

            let sale_value = item
                .sale_value
                .ok_or(ReportError::MissingData("sale_value".into()))?;

            let cost_basis = item
                .cost_basis
                .ok_or(ReportError::MissingData("cost_basis".into()))?;

            let net_proceeds = item
                .net_proceeds
                .ok_or(ReportError::MissingData("net_proceeds".into()))?;

            let average_cost_basis = item
                .average_cost_basis
                .ok_or(ReportError::MissingData("average_cost_basis".into()))?;

            let tax_base = item
                .tax_base
                .ok_or(ReportError::MissingData("tax_base".into()))?;

            let tax_amount = item
                .tax_amount
                .map(Money::from_i128)
                .ok_or(ReportError::MissingData("tax_amount".into()))?;

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
                purchase_value,
                sale_value,
                cost_basis,
                net_proceeds,
                average_cost_basis,
                tax_base,
                tax_amount,
            })
        })
        .collect();
    res
}

fn _create_combined_trade_iterator<'a>(
    df: &'a DataFrame,
) -> Result<impl Iterator<Item = AggregatedTradeItem<'a>> + 'a, ReportError> {
    let ticker_iter = df.column("ticker")?.str()?.into_iter();
    let start_iter = df.column("trade_period_start")?.i64()?.into_iter();
    let end_iter = df.column("trade_period_end")?.i64()?.into_iter();

    let buy_quantity_iter = df.column("buy_quantity")?.u32()?.into_iter();
    let sell_quantity_iter = df.column("sell_quantity")?.u32()?.into_iter();

    let buy_commission_iter = df.column("buy_commission")?.i128()?.into_iter();
    let sell_commission_iter = df.column("sell_commission")?.i128()?.into_iter();

    let purchase_value_iter = df.column("purchase_value")?.i128()?.into_iter();
    let sale_value_iter = df.column("sale_value")?.i128()?.into_iter();

    let cost_basis_iter = df.column("cost_basis")?.i128()?.into_iter();
    let net_proceeds_iter = df.column("net_proceeds")?.i128()?.into_iter();

    let average_cost_basis_iter = df.column("average_cost_basis")?.i128()?.into_iter();

    let tax_base_iter = df.column("tax_base")?.i128()?.into_iter();
    let tax_amount_iter = df.column("tax_amount")?.i128()?.into_iter();

    let combined_itr = izip!(
        ticker_iter,
        start_iter,
        end_iter,
        buy_quantity_iter,
        sell_quantity_iter,
        buy_commission_iter,
        sell_commission_iter,
        purchase_value_iter,
        sale_value_iter,
        cost_basis_iter,
        net_proceeds_iter,
        average_cost_basis_iter,
        tax_base_iter,
        tax_amount_iter
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
            purchase_value,
            sale_value,
            cost_basis,
            net_proceeds,
            average_cost_basis,
            tax_base,
            tax_amount,
        )| {
            AggregatedTradeItem {
                ticker,
                trade_period_start,
                trade_period_end,
                buy_quantity,
                sell_quantity,
                buy_commission,
                sell_commission,
                purchase_value,
                sale_value,
                cost_basis,
                net_proceeds,
                average_cost_basis,
                tax_base,
                tax_amount,
            }
        },
    );
    Ok(combined_itr)
}
