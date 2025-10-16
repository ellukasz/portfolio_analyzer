use std::path::Path;

use mbank_emakler_csv::loader;
use shared_contracts::models::money::Money;

#[test]
fn load_invalid_file_return_error() {
    let file_path = Path::new("/tests/data/this_file_does_not_exist.CSV");
    let result = loader::load(file_path);

    assert!(result.is_err(), "Expected Err, got: {result:?}");
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.starts_with("Load error: Failed to open file:"),
        "Unexpected Err: {err_msg:?}"
    );
}

#[test]
fn load_parse_file_return_trade_order() {
    let file_path = Path::new("tests/data/test.Csv");
    let result = loader::load(file_path);

    assert!(
        result.is_ok(),
        "Expected Ok for existing file, got: {result:?}"
    );

    let records = result.unwrap();
    let record = &records[0];

    assert_eq!(record.exchange, "WWA-GPW".to_string(),);

    assert_eq!(record.instrument_symbol, "AAAA".to_string(),);

    assert_eq!(record.commission, Money::from_string("5"),);
}
