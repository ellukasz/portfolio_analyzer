mod command;
mod error;
mod mbank_trade_report_csv_handler;

use crate::command::{Cli, Commands};
use clap::Parser;
use std::path::Path;

fn main() -> Result<(), error::CliError> {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::MbankTradeReportCsv { trade_orders_file } => {
            let trade_orders_path: &Path = Path::new(trade_orders_file);

            mbank_trade_report_csv_handler::handle(trade_orders_path)?;
        }
    }
    Ok(())
}
