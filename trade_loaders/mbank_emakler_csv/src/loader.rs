use crate::mapper;
use crate::model::{CSV_HEADER_FIELDS, Csv};
use csv::ReaderBuilder;
use encoding_rs_io::DecodeReaderBytesBuilder;
use shared_contracts::errors::TradeLoaderError;
use shared_contracts::models::trade_order::TradeOrder;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

pub fn load(file_path: &str) -> Result<Vec<TradeOrder>, TradeLoaderError> {
    let csv_file = File::open(file_path)
        .map_err(|e| TradeLoaderError::Load(format!("Failed to open file: {}", e)))?;

    let header_position = _find_header_position(&csv_file)?;

    let csv_model = _parse(header_position, &csv_file)?;
    let domain_model = _map(csv_model);
    return domain_model;
}

fn _find_header_position(file: &File) -> Result<csv::Position, TradeLoaderError> {
    let reader = BufReader::new(file);
    let transcoded_reader = DecodeReaderBytesBuilder::new().build(reader);

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .delimiter(b';')
        .from_reader(transcoded_reader);

    let expected_headers = CSV_HEADER_FIELDS
        .iter()
        .map(|&field| field.to_lowercase())
        .collect::<HashSet<String>>();

    for result in rdr.records() {
        let record = result
            .map_err(|e| TradeLoaderError::Parse(format!("Failed to read CSV record: {}", e)))?;
        let actual_header = record
            .iter()
            .map(|r| r.to_lowercase())
            .collect::<HashSet<String>>();
        if expected_headers == actual_header {
            return Ok(rdr.position().clone());
        }
    }

    Err(TradeLoaderError::Parse(
        "Header not found in CSV file".to_string(),
    ))
}

fn _parse(header_position: csv::Position, csv_file: &File) -> Result<Vec<Csv>, TradeLoaderError> {
    let file_reader = BufReader::new(csv_file);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file_reader);

    rdr.seek(header_position).map_err(|e| {
        TradeLoaderError::Parse(format!("Failed to seek to header position: {}", e))
    })?;

    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let csv_model: Csv = result
            .map_err(|e| TradeLoaderError::Parse(format!("Failed to parse CSV record: {}", e)))?;
        records.push(csv_model);
    }
    Ok(records)
}

fn _map(records: Vec<Csv>) -> Result<Vec<TradeOrder>, TradeLoaderError> {
    let mut orders = Vec::new();
    for record in records {
        orders.push(mapper::map(record)?);
    }
    Ok(orders)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_header_position() {
        let file_path = "tests/data/eMAKLER_historia_zlecen.Csv";
        let file = File::open(file_path).expect("Failed to open test file");
     
        let position = _find_header_position(&file).unwrap();
     
        assert_eq!(position.line(), 35);
    }
}
