use std::fs::File;
use std::path::Path;

use polars::prelude::*;
use shared_contracts::errors::PortfolioError;

pub struct CsvConf<'a> {
    pub input_file: &'a Path,
    pub output_file: &'a Path,
    pub market_data: &'a Path,
    pub investment_amount: f64,
    pub commission_percent: f64,
    pub commission_min: f64,
}

pub fn calculate(arg: &CsvConf) -> Result<(), PortfolioError> {
    let upside_df = read_df_from_csv(arg.input_file)?;
    let market_data_df = read_df_from_csv(arg.market_data)?;

    let mut df = _calculate(upside_df, market_data_df, arg)?.collect()?;
    let output = arg.output_file;

    let mut file = File::create(output)?;

    CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .finish(&mut df)?;
    Ok(())
}

fn read_df_from_csv(csv: &Path) -> Result<LazyFrame, PortfolioError> {
    let path = PlPath::Local(csv.into());
    let lf_csv = LazyCsvReader::new(path)
        .with_has_header(true)
        .with_try_parse_dates(true)
        .finish()?;
    Ok(lf_csv)
}

pub fn _calculate(
    upside_df: LazyFrame,
    market_data_df: LazyFrame,
    conf: &CsvConf,
) -> Result<LazyFrame, PortfolioError> {
    let commission_percent = lit(conf.commission_percent);
    let commission_min = lit(conf.commission_min);

    let main_df = upside_df
        .lazy()
        .left_join(market_data_df, col("instrument"), col("instrument"))
        .select([
            col("instrument"),
            col("upside"),
            col("created_by"),
            col("created_at"),
            col("max_quantity"),
            col("actual_price"),
            //calculate buy commission
            when(
                ((col("max_quantity") * col("actual_price")) * commission_percent.clone())
                    .gt(commission_min.clone()),
            )
            .then((col("max_quantity") * col("actual_price")) * commission_percent.clone())
            .otherwise(commission_min.clone())
            .alias("buy_commission"),
            //calculate sell commission
            when(
                ((col("max_quantity") * col("upside")) * commission_percent.clone())
                    .gt(commission_min.clone()),
            )
            .then((col("max_quantity") * col("upside")) * commission_percent.clone())
            .otherwise(commission_min.clone())
            .alias("sell_commission"),
        ])
        .with_columns([
            ((col("max_quantity") * col("actual_price")) + col("buy_commission"))
                .alias("cost_basis"),
            ((col("max_quantity") * col("upside")) + col("sell_commission")).alias("net_proceeds"),
        ])
        .with_columns([(col("net_proceeds") - col("cost_basis")).alias("tax_base")])
        .with_columns([(col("tax_base") * lit(0.19)).alias("tax_amount")])
        .with_columns([
            (col("net_proceeds") - col("cost_basis") - col("tax_amount")).alias("net_profit"),
        ])
        .sort(
            ["instrument", "created_by", "created_at"],
            Default::default(),
        );

    Ok(main_df)
}
