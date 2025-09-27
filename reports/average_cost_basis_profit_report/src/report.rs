use crate::data_frame_factory;
use chrono::{DateTime, Utc};
use itertools::izip;
use polars::prelude::*;
use shared_contracts::{
    errors::ReportError,
    models::trade_order::{OrderSide, TradeOrder},
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
    pub buy_gross_value: i128,
    pub sell_gross_value: i128,
    pub buy_net_value: i128,
    pub sell_net_value: i128,
    pub buy_average_value: i128,
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
                .alias("buy_gross_value"),
            when(col("order_side").eq(lit(OrderSide::Sell.to_string())))
                .then(col("price") * col("filled_quantity"))
                .otherwise(lit(0_i128))
                .sum()
                .alias("sell_gross_value"),
        ])
        .with_columns([
            (col("buy_gross_value") + col("buy_commission")).alias("buy_net_value"),
            (col("sell_gross_value") + col("sell_commission")).alias("sell_net_value"),
        ])
        .with_columns([(col("buy_net_value") / col("buy_quantity")).alias("buy_average_value")])
        .sort(["instrument_symbol"], Default::default())
        .collect()?;
    //  println!("{}", df);

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
    pub buy_gross_value: Option<i128>,
    pub sell_gross_value: Option<i128>,
    pub buy_net_value: Option<i128>,
    pub sell_net_value: Option<i128>,
    pub buy_average_value: Option<i128>,
}

fn _crete_instrument_report(df: &DataFrame) -> Result<Vec<Instrument>, ReportError> {
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

            let buy_gross_value = item
                .buy_gross_value
                .ok_or(ReportError::MissingData("buy_gross_value".into()))?;

            let sell_gross_value = item
                .sell_gross_value
                .ok_or(ReportError::MissingData("sell_gross_value".into()))?;

            let buy_net_value = item
                .buy_net_value
                .ok_or(ReportError::MissingData("buy_net_value".into()))?;
            let sell_net_value = item
                .sell_net_value
                .ok_or(ReportError::MissingData("sell_net_value".into()))?;
            let buy_average_value = item
                .buy_average_value
                .ok_or(ReportError::MissingData("buy_average_value".into()))?;

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
                buy_gross_value,
                sell_gross_value,
                buy_net_value,
                sell_net_value,
                buy_average_value,
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

    let buy_gross_value_iter = df.column("buy_gross_value")?.i128()?.into_iter();
    let sell_gross_value_iter = df.column("sell_gross_value")?.i128()?.into_iter();

    let buy_net_value_iter = df.column("buy_net_value")?.i128()?.into_iter();
    let sell_net_value_iter = df.column("sell_net_value")?.i128()?.into_iter();

    let buy_average_value_iter = df.column("buy_average_value")?.i128()?.into_iter();

    let combined_itr = izip!(
        ticker_iter,
        start_iter,
        end_iter,
        buy_quantity_iter,
        sell_quantity_iter,
        buy_commission_iter,
        sell_commission_iter,
        buy_gross_value_iter,
        sell_gross_value_iter,
        buy_net_value_iter,
        sell_net_value_iter,
        buy_average_value_iter
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
            buy_gross_value,
            sell_gross_value,
            buy_net_value,
            sell_net_value,
            buy_average_value,
        )| {
            AggregatedTradeItem {
                ticker,
                trade_period_start,
                trade_period_end,
                buy_quantity,
                sell_quantity,
                buy_commission,
                sell_commission,
                buy_gross_value,
                sell_gross_value,
                buy_net_value,
                sell_net_value,
                buy_average_value,
            }
        },
    );
    Ok(combined_itr)
}
