use thiserror::Error;

#[derive(Debug, Error)]
pub enum TradeLoaderError {
    #[error("Lod error: {0}")]
    Load(String),
    #[error("Parse error: {0}")]
    Parse(String),
}
