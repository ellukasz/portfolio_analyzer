use polars::prelude::PolarsError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum TradeLoaderError {
    #[error("Load error: {0}")]
    Load(String),
    #[error("Parse error: {0}")]
    Parse(String),
}

#[derive(Debug, Error)]
pub enum ReportError {
    #[error("Can't create report, error: {0}")]
    CalculationError(#[from] PolarsError),
    #[error("Input error: {0}")]
    InputError(String),
    #[error("General error: {0}")]
    Error(String),
}
