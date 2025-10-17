mod command;
mod error;
mod mbank_trade_report_csv_handler;
mod output;

use std::path::Path;

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

            print_profit_report(trade_orders_path, &report, output_type)?;
        }
    }
    Ok(())
}

fn print_profit_report(
    trade_orders_file: &Path,
    report: &ProfitReport,
    output_type: &str,
) -> Result<(), CliError> {
    match output_type.to_lowercase().as_str() {
        "csv" => output::csv::create(trade_orders_file, report),
        _ => Err(CliError::Io("Unsupported output type".to_string())),
    }
}
