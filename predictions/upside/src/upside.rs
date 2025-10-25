use std::fs::File;
use std::path::Path;

use polars::prelude::*;
use shared_contracts::errors::PortfolioError;

pub struct UpsideConf<'a> {
    pub upside_csv: &'a Path,
    pub output_file: &'a Path,
    pub market_data_csv: &'a Path,
    pub investment_amount: f64,
    pub commission_percent: f64,
    pub commission_min: f64,
}

pub fn calculate(arg: UpsideConf) -> Result<(), PortfolioError> {
    let upside_df = read_df_from_csv(arg.upside_csv)?;
    let market_data_df = read_df_from_csv(arg.market_data_csv)?;

    let df = _calculate(upside_df, market_data_df, &arg)?.collect()?;
    let mut selected_col = df.select([
        "instrument",
        "net_profit",
        "pct_change",
        "upside",
        "actual_price",
        "quantity",
        "created_by",
        "created_at",
    ])?;

    let output = arg.output_file;
    let mut write_to_file = File::create(output)?;

    util::polars::default_writer(&mut write_to_file)?.finish(&mut selected_col)?;

    Ok(())
}

fn read_df_from_csv(csv: &Path) -> Result<LazyFrame, PortfolioError> {
    let lf_csv = util::polars::default_lazy_reder(csv)?.finish()?;
    Ok(lf_csv)
}

fn _calculate(
    upside_df: LazyFrame,
    market_data_df: LazyFrame,
    conf: &UpsideConf,
) -> Result<LazyFrame, PortfolioError> {
    let commission_percent = lit(conf.commission_percent);
    let commission_min = lit(conf.commission_min);
    let round = 2;
    let mode = RoundMode::HalfToEven;
    let investment_amount = lit(conf.investment_amount);
    let main_df = upside_df
        .lazy()
        .left_join(market_data_df, col("instrument"), col("instrument"))
        .select([
            col("closing_price").alias("actual_price"),
            col("instrument"),
            col("upside"),
            col("created_by"),
            col("created_at"),
        ])
        .with_columns([(investment_amount / col("actual_price"))
            .round(0, RoundMode::HalfToEven)
            .alias("quantity")])
        .with_columns([
            (col("quantity") * col("actual_price"))
                .round(round, mode)
                .alias("purchase_value"),
            (col("quantity") * col("upside"))
                .round(round, mode)
                .alias("sale_value"),
        ])
        .with_columns([
            when((col("purchase_value") * commission_percent.clone()).gt(commission_min.clone()))
                .then(col("purchase_value") * commission_percent.clone())
                .otherwise(commission_min.clone())
                .round(round, mode)
                .alias("buy_commission"),
            when((col("sale_value") * commission_percent.clone()).gt(commission_min.clone()))
                .then(col("sale_value") * commission_percent.clone())
                .otherwise(commission_min.clone())
                .round(round, mode)
                .alias("sell_commission"),
        ])
        .with_columns([
            (col("purchase_value") + col("buy_commission"))
                .round(round, mode)
                .alias("cost_basis"),
            (col("sale_value") - col("sell_commission"))
                .round(round, mode)
                .alias("net_proceeds"),
        ])
        .with_columns([(col("net_proceeds") - col("cost_basis"))
            .round(round, mode)
            .alias("tax_base")])
        .with_columns([(col("tax_base") * lit(0.19))
            .round(round, mode)
            .alias("tax_amount")])
        .with_column(
            (col("net_proceeds") - col("cost_basis") - col("tax_amount"))
                .round(0, RoundMode::HalfToEven)
                .alias("net_profit"),
        )
        .with_column(
            ((col("tax_base") / col("cost_basis")) * lit(100))
                .round(round, mode)
                .alias("pct_change"),
        )
        .sort(
            ["instrument", "created_by", "created_at"],
            Default::default(),
        );

    Ok(main_df)
}
