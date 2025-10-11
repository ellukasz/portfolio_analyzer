use std::path::Path;

use shared_contracts::models::report::ProfitReport;

use crate::error::CliError;

pub fn handle(csv: &Path) -> Result<ProfitReport, CliError> {
    let orders = mbank_emakler_csv::loader::load(csv)?;
    let report = average_cost_basis_profit_report::report::create(orders)?;
    Ok(report)
}
