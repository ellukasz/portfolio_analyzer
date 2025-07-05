use std::fs::File;
use std::io::Write;
use std::error::Error;

#[test]
fn test_read_csv_file() -> Result<(), Box<dyn Error>> {
    let file_path = "tests/data/eMAKLER_historia_zlecen.CSV";
    // Call the function
    let result = read_csv_file(file_path)?;

    // Assert the result
//    assert_eq!(result.len(), 1);
    // Optionally, check the fields of TradeOrderCsv if needed
    // assert_eq!(result[0].field1, "value1");
  //  println!("Trade orders loaded successfully: {:?}", result);
    Ok(())
}

