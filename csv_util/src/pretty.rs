use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use comfy_table::{Cell, ContentArrangement, Table, presets::UTF8_FULL_CONDENSED};
use shared_contracts::models::report::ProfitReport;

pub fn save(file: &Path, report: ProfitReport) -> io::Result<()> {
    let mut t = Table::new();
    t.set_header(vec![
        "Instrument",
        "Net Profit",
        "Buy/Sell Quantity",
        "Tax",
        "Commissions",
    ])
    .set_content_arrangement(ContentArrangement::Dynamic)
    .load_preset(UTF8_FULL_CONDENSED);

    for i in report.instruments {
        t.add_row(vec![
            Cell::new(i.instrument_symbol.to_string()),
            Cell::new(i.net_profit.as_string()),
            Cell::new(format!("{}/{}", i.buy_quantity, i.sell_quantity)),
            Cell::new(i.tax_amount.as_string()),
            Cell::new(i.total_commission.as_string()),
        ]);
    }

    let metadata = format!(
        "--- Profit Report ---\n
        Trade from {} to {}\n
        Commission: {}\n
        Tax: {}\n
        Net Profit: {}\n\n",
        report.summary.trade_period.start,
        report.summary.trade_period.end,
        report.summary.commission_total.as_string(),
        report.summary.tax_amount_total.as_string(),
        report.summary.net_profit_total.as_string()
    );

    let report_string = format!("{}{}", metadata, t);

    let mut file = File::create(file)?;
    file.write_all(report_string.as_bytes())?;
    Ok(())
}
