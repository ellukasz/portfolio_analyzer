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
        #[arg(default_value_t = String::from("csv"))]
        output_type: String,
    },
}
