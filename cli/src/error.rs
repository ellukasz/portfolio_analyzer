use shared_contracts::errors::{ReportError, TradeLoaderError};
use std::fmt;

#[derive(Debug)]
pub enum CliError {
    Load(TradeLoaderError),
    Process(ReportError),
    Save(String),
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

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Load(e) => write!(f, "Trade Loading Error: {e}"), // <-- Reads the field `e`
            CliError::Process(e) => write!(f, "Report Processing Error: {e}"), // <-- Reads the field `e`
            CliError::Save(msg) => write!(f, "Data Saving Error: {msg}"), // <-- Reads the field `msg`
        }
    }
}
