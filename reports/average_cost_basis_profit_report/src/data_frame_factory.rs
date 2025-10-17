use polars::prelude::*;
use shared_contracts::errors::ReportError;
use shared_contracts::models::trade_order::*;

pub fn create_with_filled_orders(trade_orders: Vec<TradeOrder>) -> Result<DataFrame, ReportError> {
    let vectors = _convert_to_vectors(trade_orders)?;

    let series_submission_time = Series::new("submission_time".into(), vectors.submission_times);

    DataFrame::new(vec![
        Series::new("instrument_symbol".into(), vectors.instruments).into(),
        series_submission_time.into(),
        Series::new("order_side".into(), vectors.order_sides).into(),
        Series::new("filled_quantity".into(), vectors.filed_quenties).into(),
        Series::new("price".into(), vectors.prices).into(),
        Series::new("commission".into(), vectors.commissions).into(),
    ])
    .map_err(ReportError::CalculationError)
}

struct Vectors {
    instruments: Vec<String>,
    submission_times: Vec<i64>,
    order_sides: Vec<String>,
    filed_quenties: Vec<u32>,
    prices: Vec<i128>,
    commissions: Vec<i128>,
}

fn _convert_to_vectors(trade_orders: Vec<TradeOrder>) -> Result<Vectors, ReportError> {
    let mut submission_times: Vec<i64> = Vec::with_capacity(trade_orders.len());
    let mut order_sides: Vec<String> = Vec::with_capacity(trade_orders.len());
    let mut filed_quenties: Vec<u32> = Vec::with_capacity(trade_orders.len());
    let mut prices: Vec<i128> = Vec::with_capacity(trade_orders.len());
    let mut commissions: Vec<i128> = Vec::with_capacity(trade_orders.len());
    let mut instruments: Vec<String> = Vec::with_capacity(trade_orders.len());

    for order in trade_orders.iter() {
        if !matches!(
            order.status,
            OrderStatus::Filled | OrderStatus::PartiallyFilled
        ) {
            continue;
        }

        submission_times.push(order.submission_time.timestamp_nanos_opt().ok_or(
            ReportError::InputError("Can't convert submission_time to timestamp_nanos".to_string()),
        )?);

        instruments.push(order.instrument_symbol.clone());

        order_sides.push(order.order_side.to_string());
        filed_quenties.push(order.filled_quantity);

        prices.push(order.price.map_or(0_i128, |p| p.as_i128()));
        commissions.push(order.commission.as_i128());
    }

    let res = Vectors {
        instruments,
        submission_times,
        order_sides,
        filed_quenties,
        prices,
        commissions,
    };
    Ok(res)
}
