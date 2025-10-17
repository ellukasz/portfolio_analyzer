use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use csv::Writer;
use shared_contracts::models::report::{Instrument, ProfitReport, Summary};

use crate::error::CliError;

pub fn create(trade_orders_file: &Path, report: &ProfitReport) -> Result<(), CliError> {
    let output = &output_file(trade_orders_file, "profit_report")?;

    let mut file = File::create(output)
        .map_err(|e| CliError::Io(format!("Failed to open file to writer: {e}")))?;

    add_metadata(&mut file, &report.summary)?;

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(file);

    add_instrument_summary(&mut wtr, &report.instruments)?;
    wtr.flush()
        .map_err(|e| CliError::Io(format!("Failed to flush file: {e}")))?;

    Ok(())
}
fn add_instrument_summary(
    wtr: &mut Writer<File>,
    instruments: &Vec<Instrument>,
) -> Result<(), CliError> {
    let len = 10;
    let m_l = 10;
    wtr.write_record([
        &pad_string("Instrument", len),
        &pad_string("Net Profit", m_l),
        &pad_string("Buy Total", m_l),
        &pad_string("Sell Total", m_l),
        &pad_string("Buy Quan", m_l),
        &pad_string("Sell Quan", m_l),
        &pad_string("Tax", m_l),
        &pad_string("Commission", m_l),
    ])
    .map_err(|e| CliError::Io(format!("Failed to write header: {e}")))?;

    for instrument in instruments {
        wtr.write_record([
            &pad_string(&instrument.instrument_symbol, len),
            &pad_string(&instrument.net_profit.as_string(), m_l),
            &pad_string(&instrument.purchase_value.as_string(), m_l),
            &pad_string(&instrument.net_proceeds.as_string(), m_l),
            &pad_string(&instrument.buy_quantity.to_string(), m_l),
            &pad_string(&instrument.sell_quantity.to_string(), m_l),
            &pad_string(&instrument.tax_amount.as_string(), m_l),
            &pad_string(&instrument.total_commission.as_string(), m_l),
        ])
        .map_err(|e| CliError::Io(format!("Failed to write instrument record: {e}")))?;
    }
    Ok(())
}

fn pad_string(s_value: &str, length_value: usize) -> String {
    format!("{s_value:<length_value$}")
}

fn add_metadata(wtr: &mut File, summary: &Summary) -> Result<(), CliError> {
    let start_date = summary.trade_period.start.format("%d.%m.%Y").to_string();
    wtr.write_fmt(format_args!("#Trade Start,{}\n", &start_date))?;

    let end_date = summary.trade_period.end.format("%d.%m.%Y").to_string();
    wtr.write_fmt(format_args!("#Trade Start,{}\n\n", &end_date))?;

    wtr.write_fmt(format_args!(
        "#Commission Total,{}\n",
        &summary.commission_total.as_string()
    ))?;
    wtr.write_fmt(format_args!(
        "#Tax Total,{}\n",
        &summary.tax_amount_total.as_string()
    ))?;
    wtr.write_fmt(format_args!(
        "#Net Profit,{}\n\n",
        &summary.net_profit_total.as_string()
    ))?;

    Ok(())
}

fn output_file(trade_orders_file: &Path, suffix: &str) -> Result<PathBuf, CliError> {
    let dir = trade_orders_file
        .parent()
        .ok_or(CliError::Io("Invalid file path".to_string()));

    let file_name = trade_orders_file
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or(CliError::Io("Invalid file name".to_string()))?;

    let mut path = dir?.to_path_buf();
    path.push(format!("{file_name}_{suffix}.csv"));
    Ok(path)
}
