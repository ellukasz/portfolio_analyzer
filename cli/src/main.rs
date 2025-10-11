mod command;
mod error;
mod mbank_trade_report_csv_handler;

use std::path::{Path, PathBuf};

use clap::Parser;
use shared_contracts::models::report::ProfitReport;

use crate::{
    command::{Cli, Commands},
    error::CliError,
};

fn main() -> Result<(), error::CliError> {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::MbankTradeReportCsv {
            trade_orders_file,
            output_type,
        } => {
            let trade_orders_path: &Path = Path::new(trade_orders_file);

            let report = mbank_trade_report_csv_handler::handle(trade_orders_path)?;

            print_profit_report(trade_orders_path, report, output_type)?;
        }
    }
    Ok(())
}

fn print_profit_report(
    trade_orders_file: &Path,
    report: ProfitReport,
    output_type: &str,
) -> Result<(), CliError> {
    let output = &output_file(trade_orders_file, "_profit_report.csv")?;

    match output_type.to_lowercase().as_str() {
        "csv" => {
            let mut wtr = csv::Writer::from_path(output)
                .map_err(|e| CliError::Save(format!("Failed to create CSV writer: {e}")))?;

            wtr.serialize(&report)
                .map_err(|e| CliError::Save(format!("Failed to write CSV data: {e}")))?;

            wtr.flush()
                .map_err(|e| CliError::Save(format!("Failed to flush CSV data: {e}")))?;

            println!("Profit report saved to: {}", output.display());
            Ok(())
        }
        _ => Err(CliError::Save("Unsupported output type".to_string())),
    }
}

fn output_file(trade_orders_file: &Path, suffix: &str) -> Result<PathBuf, CliError> {
    let dir = trade_orders_file
        .parent()
        .ok_or(error::CliError::Save("Invalid file path".to_string()));

    let file_name = trade_orders_file
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or(error::CliError::Save("Invalid file name".to_string()))?;

    let mut path = dir?.to_path_buf();
    path.push(format!("{file_name}_{suffix}.csv"));
    Ok(path)
}
