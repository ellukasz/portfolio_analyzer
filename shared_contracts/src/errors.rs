use polars::prelude::PolarsError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PortfolioError {
    #[error("{0}")]
    CalculationError(#[from] PolarsError),

    #[error("{0}")]
    IO(#[from] std::io::Error),

    #[error("{0}")]
    Csv(#[from] csv::Error),

    #[error("{0}")]
    Xls(String),

    #[error("Input error: {0}")]
    InputError(String),
    #[error("General error: {0}")]
    Error(String),

    #[error("Invalid field {0}")]
    InvalidValue(String),
}
