use mbank_emakler_csv::loader;
use shared_contracts::errors::TradeLoaderError;
use shared_contracts::models::trade_order;

#[test]
fn load_invalid_file_return_error() {
    let file_path = "trade_loaders/mbank_emakler_csv/tests/data/this_file_does_not_exist.CSV";
    let result = loader::load(file_path);

    assert!(result.is_err(), "Expected Err, got: {:?}", result);
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.starts_with("Load error: Failed to open file:"),
        "Unexpected Err: {}",
        err_msg
    );
}

//#[test]
fn load_parse_file_return_trade_order() {
    let file_path = "./trade_loaders/mbank_emakler_csv/tests/data/eMAKLER_historia_zlecen.Csv";
    let result = loader::load(file_path);

    assert!(
        result.is_ok(),
        "Expected Ok for existing file, got: {:?}",
        result
    );

    let records = result.unwrap();
    // Optionally, check the number of records or fields
    // assert_eq!(records.len(), expected_len);
    // assert_eq!(records[0].stan, "expected_value");
}
