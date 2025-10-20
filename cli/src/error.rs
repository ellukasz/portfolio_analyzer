use shared_contracts::errors::PortfolioError;
use std::fmt;
use std::io;
#[derive(Debug)]
pub enum CliError {
    Process(PortfolioError),
    Io(String),
}

impl From<PortfolioError> for CliError {
    fn from(e: PortfolioError) -> Self {
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
            CliError::Process(e) => write!(f, "Report Processing Error: {e}"),
            CliError::Io(msg) => write!(f, "Data Saving Error: {msg}"),
        }
    }
}
