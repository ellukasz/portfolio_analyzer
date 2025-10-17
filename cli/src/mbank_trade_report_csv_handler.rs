use std::path::{Path, PathBuf};

use shared_contracts::models::report::ProfitReport;

use crate::error::CliError;

pub fn handle(csv: &Path) -> Result<ProfitReport, CliError> {
    let normalized_csv = tmp_path(csv, "normalized")?;
    normalized_csv
        .exists()
        .then(|| std::fs::remove_file(&normalized_csv));

    let orders = mbank_emakler_csv::loader::load(csv)?;

    csv_util::csv::save(&normalized_csv, orders.clone())
        .map_err(|e| CliError::Io(format!("Failed to save normalized CSV: {e}")))?;

    let report = average_cost_basis_profit_report::report::from_csv(normalized_csv.as_path())?;
    Ok(report)
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
