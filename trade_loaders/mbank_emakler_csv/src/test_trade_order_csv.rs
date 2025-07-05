use shared_contracts::errors::TradeLoaderError;
use crate::model::Csv;
use crate::read::read_csv;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_csv_file_not_exists() {
        let file_path = "trade_loaders/mbank_emakler_csv/tests/data/this_file_does_not_exist.CSV";
        let result = read_csv(file_path);
        
        assert!(result.is_err(), "Expected error when file does not exist, got: {:?}", result);
        
        if let Err(e) = result {
            assert!(format!("{}", e).contains("Failed to read CSV file"));
        }
    }

    #[test]
    fn test_read_csv_file_exists() {
        let file_path = "trade_loaders/mbank_emakler_csv/tests/data/eMAKLER_historia_zlecen.CSV";
        let result = read_csv(file_path);
        
        assert!(result.is_ok(), "Expected Ok for existing file, got: {:?}", result);
        
        let records = result.unwrap();
        // Optionally, check the number of records or fields
        // assert_eq!(records.len(), expected_len);
        // assert_eq!(records[0].stan, "expected_value");
    }
}

