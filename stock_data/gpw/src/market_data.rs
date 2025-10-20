use calamine::{RangeDeserializer, Reader, Xls, open_workbook};
use serde::{Deserialize, Serialize};
use shared_contracts::errors::PortfolioError;
use std::path::Path;

pub fn convert_xls_to_csv(market_data_file: &Path, output: &Path) -> Result<(), PortfolioError> {
    let records = read_xls(market_data_file)?;

    let mut wtr = csv::Writer::from_path(output)?;

    for record in records {
        wtr.serialize(record)?;
    }

    wtr.flush()?;
    Ok(())
}

fn read_xls(stocks_data_file: &Path) -> Result<Vec<MarketDataRecord>, PortfolioError> {
    let mut workbook: Xls<_> = open_workbook(stocks_data_file).map_err(|e| {
        PortfolioError::Xls(format!(
            "Invalid xls file:{}, reason:{}",
            stocks_data_file.display(),
            e
        ))
    })?;

    let sheet_range = workbook.worksheet_range("Worksheet").map_err(|e| {
        PortfolioError::Xls(format!(
            "Invalid worksheet file:{}, reason:{}",
            stocks_data_file.display(),
            e
        ))
    })?;

    let iter: RangeDeserializer<_, MarketDataRecord> = sheet_range.deserialize().map_err(|e| {
        PortfolioError::Xls(format!(
            "Deserialization error,  xls file:{}, reason:{}",
            stocks_data_file.display(),
            e
        ))
    })?;

    let mut records: Vec<MarketDataRecord> = Vec::new();

    for record in iter {
        let r = record.map_err(|e| PortfolioError::Xls(format!("Invalid data, reason:{}", e)))?;
        records.push(r);
    }

    Ok(records)
}

#[derive(Debug, Serialize, Deserialize)]
struct MarketDataRecord {
    #[serde(rename = "date")]
    pub date: String,

    #[serde(rename = "instrument")]
    pub name: String,

    #[serde(rename = "isin")]
    pub isin: String,

    #[serde(rename = "currency")]
    pub currency: String,

    #[serde(rename = "opening_price")]
    pub open_price: f64,

    #[serde(rename = "max_price")]
    pub high_price: f64,

    #[serde(rename = "min_price")]
    pub low_price: f64,

    #[serde(rename = "closing_price")]
    pub close_price: f64,

    #[serde(rename = "change")]
    pub change: f64,

    #[serde(rename = "volumen")]
    pub volume: i64,

    #[serde(rename = "transactions_number")]
    pub num_transactions: i64,

    #[serde(rename = "trading_volume")]
    pub turnover: f64,

    #[serde(rename = "open_positions_count")]
    pub open_positions_count: i64,

    #[serde(rename = "open_positions_value")]
    pub open_positions_value: f64,

    #[serde(rename = "par_value")]
    pub nominal_price: f64,
}
