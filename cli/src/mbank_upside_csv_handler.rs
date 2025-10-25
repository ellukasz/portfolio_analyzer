use std::path::Path;

use crate::error::CliError;

pub fn handle(
    investment_amount: f64,
    upside_csv: &Path,
    market_data_xls: &Path,
) -> Result<(), CliError> {
    let market_data_csv = util::file::new_file_with_suffix(market_data_xls, "normalized.csv")?;
    gpw::market_data::convert_xls_to_csv(market_data_xls, market_data_csv.as_path())?;

    let upside_report_csv = util::file::new_file_with_suffix(upside_csv, "report.csv")?;

    let conf = upside::upside::UpsideConf {
        upside_csv,
        output_file: &upside_report_csv,
        market_data_csv: &market_data_csv,
        investment_amount,
        commission_percent: 0.039_f64,
        commission_min: 5_f64,
    };
    upside::upside::calculate(conf)?;
    Ok(())
}
