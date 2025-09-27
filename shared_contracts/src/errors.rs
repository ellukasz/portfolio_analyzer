use polars::prelude::PolarsError;
use rust_decimal::Error as DecimalError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum TradeLoaderError {
    #[error("Load error: {0}")]
    Load(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Money conversion error: {0}")]
    MoneyConversionError(#[from] DecimalError),
}

#[derive(Debug, Error)]
pub enum ReportError {
    #[error("Can't create report, error: {0}")]
    CalculationError(#[from] PolarsError),
    #[error("Input error: {0}")]
    InputError(String),
    #[error("General error: {0}")]
    Error(String),
    #[error("Missing data: {0}")]
    MissingData(String),
}
