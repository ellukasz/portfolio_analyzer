use std::{
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

pub fn new_file_with_suffix(trade_orders_file: &Path, suffix: &str) -> Result<PathBuf, io::Error> {
    let dir = trade_orders_file.parent().ok_or_else(|| {
        io::Error::new(
            ErrorKind::InvalidInput,
            "Invalid file path: could not determine parent directory",
        )
    })?;

    let file_name = trade_orders_file
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| {
            io::Error::new(
                ErrorKind::InvalidInput,
                "Invalid file name: could not get valid filename stem",
            )
        })?;

    let mut path = dir.to_path_buf();
    path.push(format!("{file_name}_{suffix}"));

    Ok(path)
}
