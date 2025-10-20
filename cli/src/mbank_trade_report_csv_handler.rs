use crate::error::CliError;
use std::path::{Path, PathBuf};

pub fn handle(csv: &Path) -> Result<(), CliError> {
    let normalized_orders_csv = tmp_path(csv, "normalized")?;

    normalized_orders_csv
        .exists()
        .then(|| std::fs::remove_file(&normalized_orders_csv));

    mbank_emakler_csv::loader::normalize(csv, normalized_orders_csv.as_path())?;

    let portfolio_csv = tmp_path(csv, "portfolio")?;

    average_cost_basis_profit_report::report::calculate_and_save(
        normalized_orders_csv.as_path(),
        portfolio_csv.as_path(),
    )?;

    Ok(())
}

fn tmp_path(trade_orders_file: &Path, suffix: &str) -> Result<PathBuf, CliError> {
    let dir = trade_orders_file
        .parent()
        .ok_or(CliError::Io("Invalid file path".to_string()));

    let file_name = trade_orders_file
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or(CliError::Io("Invalid file name".to_string()))?;

    let mut path = dir?.to_path_buf();
    path.push(format!("{file_name}_{suffix}.csv"));
    Ok(path)
}
