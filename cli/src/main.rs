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
    let file = output_file(trade_orders_file, "_report.txt")?;

    match output_type.to_lowercase().as_str() {
        "pretty" => {
            csv_util::pretty::save(&file, report)?;
            Ok(())
        }
        _ => Err(CliError::Io("Unsupported output type".to_string())),
    }
}

fn output_file(trade_orders_file: &Path, suffix: &str) -> Result<PathBuf, CliError> {
    let dir = trade_orders_file
        .parent()
        .ok_or(CliError::Io("Invalid file path".to_string()));

    let file_name = trade_orders_file
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or(CliError::Io("Invalid file name".to_string()))?;

    let mut path = dir?.to_path_buf();
    path.push(format!("{file_name}_{suffix}"));
    Ok(path)
}
