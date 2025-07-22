use crate::mapper;
use crate::model::{CSV_HEADER_FIELDS, Csv};
use csv::{Reader, ReaderBuilder, StringRecord};
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};
use shared_contracts::errors::TradeLoaderError;
use shared_contracts::models::trade_order::TradeOrder;
use std::collections::HashSet;
use std::fs::{File, ReadDir};
use std::io::{BufReader, Seek, SeekFrom};

pub fn load(file_path: &str) -> Result<Vec<TradeOrder>, TradeLoaderError> {
    let header_position = _find_header_position(file_path)?;

    let csv_model = _parse_from_header(header_position, file_path)?;

    let domain_model = _map(csv_model);

    return domain_model;
}

fn _new_csv_reader(
    file_path: &str,
    position: Option<csv::Position>,
    has_header: bool,
    flexible: bool,
) -> Result<Reader<DecodeReaderBytes<BufReader<File>, Vec<u8>>>, TradeLoaderError> {
    let file = File::open(file_path).map_err(|e| {
        TradeLoaderError::Load(format!("can't open file:{}, err:{} ", file_path, e))
    })?;

    let mut reader = BufReader::new(file);
    if position.is_some() {
        reader
            .seek(SeekFrom::Start(position.unwrap().byte()))
            .map_err(|e| {
                TradeLoaderError::Parse(format!(
                    "failed to set file position file: {}, err:{}",
                    file_path, e
                ))
            })?;
    }

    let transcoded_reader = DecodeReaderBytesBuilder::new().build(reader);

    let rdr = ReaderBuilder::new()
        .has_headers(has_header)
        .flexible(flexible)
        .delimiter(b';')
        .from_reader(transcoded_reader);

    Ok(rdr)
}

fn _find_header_position(path: &str) -> Result<csv::Position, TradeLoaderError> {
    let rdr = _new_csv_reader(path, None, false, true)?;

    let expected_headers = CSV_HEADER_FIELDS
        .iter()
        .map(|&field| field.to_lowercase())
        .collect::<HashSet<String>>();

    let mut iter = rdr.into_records();

    loop {
        let pos = iter.reader().position().clone();
        let record: Option<Result<StringRecord, csv::Error>> = iter.next();

        if record.is_none() {
            return Err(TradeLoaderError::Parse(format!(
                "csv file: {} does not contain a header",
                path
            )));
        }

        let normalized_record = record
            .unwrap()
            .unwrap()
            .iter()
            .map(|r| r.to_lowercase())
            .collect::<HashSet<String>>();

        if expected_headers == normalized_record {
            return Ok(pos);
        }
    }
}

fn _parse_from_header(
    header_position: csv::Position,
    path: &str,
) -> Result<Vec<Csv>, TradeLoaderError> {
    let mut rdr = _new_csv_reader(path, Some(header_position), true, false)?;

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
    fn parse() {
        let file_path = "tests/data/eMAKLER_historia_zlecen.Csv";

        let position = _find_header_position(file_path).unwrap();

        let r = _parse_from_header(position, file_path).unwrap();
        print!("Parsed records: {:?}", r);
        assert_eq!(r.len(), 31);
    }
}
