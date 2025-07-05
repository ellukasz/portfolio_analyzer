use shared_contracts::errors::TradeLoaderError;
use crate::model::Csv;

fn read_csv_file(file_path: &str) -> Result<Vec<Csv>, Box<TradeLoaderError>> {
    let mut rdr = csv::Reader::from_path(file_path)
        .map_err(|e| TradeLoaderError::Load(format!("Failed to read CSV file: {}", e)))?;

        let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: Csv = result
            .map_err(|e| TradeLoaderError::Parse(format!("Failed to parse CSV record: {}", e)))?;
        
            records.push(record);
    }

    Ok(records)
}
