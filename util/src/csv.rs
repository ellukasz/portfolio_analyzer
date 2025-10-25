use std::{fs::File, path::Path};

use csv::{Writer, WriterBuilder};
use shared_contracts::errors::PortfolioError;

pub fn default_writer(path: &Path) -> Result<Writer<File>, PortfolioError> {
    let w = WriterBuilder::new().delimiter(b';').from_path(path)?;
    Ok(w)
}
