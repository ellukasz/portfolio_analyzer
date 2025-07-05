use shared_contracts::errors::TradeLoaderError;
use crate::model::Csv;

pub fn read_csv(file_path: &str) -> Result<Vec<Csv>, Box<TradeLoaderError>> {
    let mut rdr = csv::Reader::from_path(file_path)
        .map_err(|e| TradeLoaderError::Load(format!("Failed to read CSV file: {}", e)))?;

    return _parse(&mut rdr)
}

fn _goto_headers(rdr: &mut csv::Reader<std::fs::File>) -> Result<(), Box<TradeLoaderError>> {
}

fn _parse(rdr: &mut csv::Reader<std::fs::File>) -> Result<Vec<Csv>, Box<TradeLoaderError>> {
    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: Csv = result
            .map_err(|e| TradeLoaderError::Parse(format!("Failed to parse CSV record: {}", e)))?;
        records.push(record);
    }

    Ok(records)
}