use thiserror::Error;

#[derive(Debug, Error,PartialEq)]
pub enum TradeLoaderError {
    #[error("Load error: {0}")]
    Load(String),
    #[error("Parse error: {0}")]
    Parse(String),
}
