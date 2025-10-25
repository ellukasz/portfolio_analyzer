use calamine::{RangeDeserializer, Reader, Xls, open_workbook};
use serde::{Deserialize, Serialize};
use shared_contracts::errors::PortfolioError;
use std::path::Path;

pub fn convert_xls_to_csv(market_data_file: &Path, output: &Path) -> Result<(), PortfolioError> {
    let records = read_xls(market_data_file)?;
    write_to_csv(records, output)?;
    Ok(())
}

fn write_to_csv(records: Vec<MarketDataRecord>, output: &Path) -> Result<(), PortfolioError> {
    let mut wtr = util::csv::default_writer(output)?;

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
    #[serde(alias = "Data", rename = "date")]
    pub date: String,

    #[serde(alias = "Nazwa", rename = "instrument")]
    pub name: String,

    #[allow(dead_code)]
    #[serde(alias = "ISIN", skip_serializing)]
    pub isin: String,

    #[allow(dead_code)]
    #[serde(alias = "Waluta", skip_serializing)]
    pub currency: String,

    #[serde(alias = "Kurs otwarcia", rename = "opening_price")]
    pub open_price: f64,

    #[serde(alias = "Kurs max", rename = "max_price")]
    pub high_price: f64,

    #[serde(alias = "Kurs min", rename = "min_price")]
    pub low_price: f64,

    #[serde(alias = "Kurs zamknięcia", rename = "closing_price")]
    pub close_price: f64,

    #[allow(dead_code)]
    #[serde(alias = "Zmiana", skip_serializing)]
    pub change: f64,

    #[serde(alias = "Wolumen", rename = "volume")]
    pub volume: i64,

    #[serde(alias = "Liczba Transakcji", rename = "transaction_number")]
    pub num_transactions: i64,

    #[allow(dead_code)]
    #[serde(alias = "Obrót", skip_serializing)]
    pub turnover: f64,

    #[serde(alias = "Liczba otwartych pozycji", rename = "open_position_number")]
    pub open_positions_count: i64,

    #[allow(dead_code)]
    #[serde(alias = "Wartość otwartych pozycji", skip_serializing)]
    pub open_positions_value: f64,

    #[allow(dead_code)]
    #[serde(alias = "Cena nominalna", skip_serializing)]
    pub nominal_price: f64,
}
