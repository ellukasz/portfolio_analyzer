use crate::mapper;
use crate::model::{Csv, HEADER};
use encoding_rs::WINDOWS_1250;
use shared_contracts::errors::PortfolioError;
use shared_contracts::models::trade_order::TradeOrder;
use std::fs;
use std::io::Cursor;
use std::path::Path;

pub fn normalize(input: &Path, output: &Path) -> Result<(), PortfolioError> {
    let records = load(input)?;

    let mut wtr = csv::Writer::from_path(output)?;

    for record in records {
        wtr.serialize(record)?;
    }
    wtr.flush()?;

    Ok(())
}

fn load(file_path: &Path) -> Result<Vec<TradeOrder>, PortfolioError> {
    let full_input = decode_windows1250(file_path)?;
    let csv_data_bytes = remove_metadata(full_input)?;
    let csv_model = parse(csv_data_bytes)?;
    let orders = map(csv_model)?;
    Ok(orders)
}

fn decode_windows1250(file: &Path) -> Result<String, PortfolioError> {
    let bytes = fs::read(file)?;

    let (full_input, _, malformed_content) = WINDOWS_1250.decode(&bytes);

    if malformed_content {
        return Err(PortfolioError::Error(format!(
            "file {file:?} contains malformed content that cannot be decoded as Windows-1250"
        )));
    }
    Ok(full_input.into_owned())
}

fn remove_metadata(csv: String) -> Result<Vec<u8>, PortfolioError> {
    let mut header_found = false;
    let mut csv_data_bytes: Vec<u8> = Vec::new();

    for line in csv.lines() {
        let cleaned_line2 = line.replace('\u{a0}', " ");
        let cleaned_line = cleaned_line2.trim();

        if header_found {
            csv_data_bytes.extend_from_slice(cleaned_line.as_bytes());
            csv_data_bytes.push(b'\n');
        } else if line == HEADER {
            csv_data_bytes.extend_from_slice(cleaned_line.as_bytes());
            csv_data_bytes.push(b'\n');
            header_found = true;
        }
    }
    if !header_found {
        return Err(PortfolioError::Error(format!(
            "Can't find header: {HEADER:?} in file"
        )));
    }
    Ok(csv_data_bytes)
}

fn parse(csv_data_bytes: Vec<u8>) -> Result<Vec<Csv>, PortfolioError> {
    let csv_stream = Cursor::new(csv_data_bytes);

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .flexible(false)
        .from_reader(csv_stream);

    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: Csv = result?;
        records.push(record);
    }
    Ok(records)
}

fn map(records: Vec<Csv>) -> Result<Vec<TradeOrder>, PortfolioError> {
    let mut orders = Vec::new();
    for record in records {
        orders.push(mapper::map(record)?);
    }
    Ok(orders)
}
