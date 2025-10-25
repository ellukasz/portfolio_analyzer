use crate::error::CliError;
use std::path::Path;

pub fn handle(csv: &Path) -> Result<(), CliError> {
    let normalized_orders_csv = util::file::new_file_with_suffix(csv, "normalized.csv")?;

    normalized_orders_csv
        .exists()
        .then(|| std::fs::remove_file(&normalized_orders_csv));

    mbank_emakler_csv::loader::normalize(csv, normalized_orders_csv.as_path())?;

    let portfolio_csv = util::file::new_file_with_suffix(csv, "portfolio.csv")?;

    average_cost_basis_profit_report::report::calculate_and_save(
        normalized_orders_csv.as_path(),
        portfolio_csv.as_path(),
    )?;

    Ok(())
}
