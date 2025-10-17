use shared_contracts::models::trade_order::TradeOrder;
use std::io;
use std::path::Path;

pub fn save(output: &Path, trade_order: Vec<TradeOrder>) -> Result<(), io::Error> {
    let mut wtr = csv::Writer::from_path(output)?;

    for order in trade_order {
        wtr.serialize(order)?;
    }

    wtr.flush()?;

    Ok(())
}
