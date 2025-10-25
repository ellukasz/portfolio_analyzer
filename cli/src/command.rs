use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    MbankTradeReportCsv {
        trade_orders_file: String,
    },
    UpsideCSV {
        investment_amount: f64,
        upside_report: String,
        market_data: String,
    },
}
