use shared_contracts::errors::{ReportError, TradeLoaderError};
use std::fmt;
use std::io;
#[derive(Debug)]
pub enum CliError {
    Load(TradeLoaderError),
    Process(ReportError),
    Io(String),
}

impl From<TradeLoaderError> for CliError {
    fn from(e: TradeLoaderError) -> Self {
        CliError::Load(e)
    }
}

impl From<ReportError> for CliError {
    fn from(e: ReportError) -> Self {
        CliError::Process(e)
    }
}
impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        CliError::Io(format!("I/O operation failed: {err}"))
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Load(e) => write!(f, "Trade Loading Error: {e}"),
            CliError::Process(e) => write!(f, "Report Processing Error: {e}"),
            CliError::Io(msg) => write!(f, "Data Saving Error: {msg}"),
        }
    }
}
