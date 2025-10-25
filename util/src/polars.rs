use std::{fs::File, path::Path};

use polars::prelude::CsvWriter;
use polars::prelude::*;
use shared_contracts::errors::PortfolioError;

pub fn default_lazy_reder(path: &Path) -> Result<LazyCsvReader, PortfolioError> {
    let p = path.to_string_lossy().to_string();

    let df_csv = LazyCsvReader::new(PlPath::from_string(p))
        .with_has_header(true)
        .with_separator(b';')
        .with_try_parse_dates(true);
    Ok(df_csv)
}

pub fn default_writer(path: &mut File) -> Result<CsvWriter<&mut File>, PortfolioError> {
    let writer = CsvWriter::new(path)
        .include_header(true)
        .with_separator(b';');
    Ok(writer)
}
