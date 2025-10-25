mod command;
mod error;
mod mbank_trade_report_csv_handler;
mod mbank_upside_csv_handler;
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
        Commands::UpsideCSV {
            investment_amount,
            upside_report,
            market_data,
        } => {
            let upside_report_path = Path::new(upside_report);
            let maket_data_path = Path::new(market_data);

            mbank_upside_csv_handler::handle(
                *investment_amount,
                upside_report_path,
                maket_data_path,
            )?;
        }
    }
    Ok(())
}
