use super::model::Csv;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Europe::Warsaw;
use rust_decimal::Decimal;
use shared_contracts::errors::PortfolioError;
use shared_contracts::models::money::Money;
use shared_contracts::models::trade_order::{
    InstrumentType, OrderSide, OrderStatus, OrderType, TradeOrder,
};
use std::str::FromStr;

pub(super) fn map(record: Csv) -> Result<TradeOrder, PortfolioError> {
    let order = TradeOrder {
        instrument: record.instrument_symbol.clone(),
        instrument_type: InstrumentType::Stock,
        order_type: _map_order_type(&record)?,
        order_side: _map_side(&record.side)?,
        quantity: _map_i64(&record.quantity, "quantity")?,
        filled_quantity: _map_i64(&record.filled_quantity, "filled_quantity")?,
        price: _map_price(&record)?,
        commission: _map_commission(&record)?,
        status: _map_status(&record.status)?,
        submission_time: _map_warsaw_time_to_utc(&record.order_date)?,
        currency: record.currency,
        exchange: record.exchange,
    };
    Ok(order)
}
fn _map_i64(value: &str, field_name: &str) -> Result<i64, PortfolioError> {
    value
        .trim()
        .parse::<i64>()
        .map_err(|e| PortfolioError::Error(format!("Invalid {field_name:?} value {value:?}: {e}")))
}
fn _map_status(status: &str) -> Result<OrderStatus, PortfolioError> {
    match status.trim().to_lowercase().as_str() {
        "przyjęte" => Ok(OrderStatus::Pending),
        "zamknięte" => Ok(OrderStatus::Closed),
        "zrealizowane" => Ok(OrderStatus::Filled),
        "anulowane" => Ok(OrderStatus::Cancelled),
        "odrzucone" => Ok(OrderStatus::Rejected),
        _ => Err(PortfolioError::Error(format!(
            "Unknown order status: {status:?}"
        ))),
    }
}

fn _map_warsaw_time_to_utc(date_raw: &str) -> Result<DateTime<Utc>, PortfolioError> {
    let date = NaiveDateTime::parse_from_str(date_raw, "%d.%m.%Y %H:%M:%S")
        .map_err(|e| PortfolioError::Error(format!("Invalid date {date_raw:?}: {e}")))?;

    match Warsaw.from_local_datetime(&date) {
        chrono::offset::LocalResult::Single(dt) => Ok(dt.with_timezone(&Utc)),
        chrono::offset::LocalResult::Ambiguous(_dt1, _dt2) => Err(PortfolioError::Error(
            format!("Unknown date time: {date:?}").to_string(),
        )),
        chrono::offset::LocalResult::None => Err(PortfolioError::Error(
            format!("Unknown date time: {date:?}").to_string(),
        )),
    }
}

fn _map_side(side: &str) -> Result<OrderSide, PortfolioError> {
    match side.trim().to_uppercase().as_str() {
        "K" => Ok(OrderSide::Buy),
        "S" => Ok(OrderSide::Sell),
        _ => Err(PortfolioError::Error(
            format!("Unknown order side, side:{side:?}").to_string(),
        )),
    }
}

fn _map_order_type(record: &Csv) -> Result<OrderType, PortfolioError> {
    if !record.activation_limit.is_empty() {
        Ok(OrderType::StopLimit)
    } else if !record.price_limit.is_empty() {
        Ok(OrderType::Limit)
    } else {
        Err(PortfolioError::Error(
            format!("Unknown order type, record:{record:?}").to_string(),
        ))
    }
}

fn _map_price(record: &Csv) -> Result<Option<Money>, PortfolioError> {
    if record.price_limit.is_empty() {
        return Ok(None);
    }

    let price = Money::from_string(&record.price_limit);
    Ok(Some(price))
}

fn _map_commission(record: &Csv) -> Result<Money, PortfolioError> {
    let filled_quantity_raw = &record.filled_quantity;
    let fulfilled_quantity: Decimal =
        Decimal::from_str_exact(filled_quantity_raw).map_err(|e| {
            PortfolioError::Error(format!(
                "Invalid filled_quantity value {filled_quantity_raw:?}: {e}"
            ))
        })?;
    let price_limit: Decimal = Money::from_string(&record.price_limit).as_decimal();

    let mbank_percentage: Decimal = Decimal::from_str("0.039")?;

    let mbank_thrashold = Decimal::from_str("5.00")?;

    let commission = (price_limit * fulfilled_quantity) * mbank_percentage;

    if commission < mbank_thrashold {
        Ok(Money::from_decimal(mbank_thrashold))
    } else {
        Ok(Money::from_decimal(commission))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use shared_contracts::models::trade_order::OrderType;

    #[test]
    fn map_order_type_activation_limit_empty_return_limit() {
        let record = _record_with_price_limit("10", "");

        let actual_order_type = _map_order_type(&record);
        let expected_order_type: OrderType = OrderType::Limit;
        assert_eq!(actual_order_type.unwrap(), expected_order_type);
    }

    #[test]
    fn map_order_type_activation_limit_not_empty_return_stop_limit() {
        let record = _record_with_price_limit("10", "9");

        let actual_order_type = _map_order_type(&record);
        let expected_order_type: OrderType = OrderType::StopLimit;

        assert_eq!(actual_order_type.unwrap(), expected_order_type);
    }

    #[test]
    fn map_warsaw_time_to_utc_order_date_not_empty_return_utc() {
        let record = _default_record();
        let actual_date_time = _map_warsaw_time_to_utc(&record.order_date).unwrap();
        let expected_date_time = Utc.with_ymd_and_hms(2025, 7, 14, 17, 20, 25).unwrap();

        assert_eq!(actual_date_time, expected_date_time);
    }

    fn _record_with_price_limit(price_limit: &str, activation_limit: &str) -> Csv {
        Csv {
            price_limit: price_limit.to_string(),
            activation_limit: activation_limit.to_string(),
            instrument_symbol: "AAPL".to_string(),
            exchange: "NASDAQ".to_string(),
            side: "B".to_string(),
            quantity: "100".to_string(),
            filled_quantity: "100".to_string(),
            status: "Zrealizowane".to_string(),
            currency: "USD".to_string(),
            order_date: "01.05.2025 15:09:10".to_string(),
        }
    }

    fn _default_record() -> Csv {
        Csv {
            price_limit: "10.00".to_string(),
            activation_limit: "".to_string(),
            instrument_symbol: "AAPL".to_string(),
            exchange: "NASDAQ".to_string(),
            side: "B".to_string(),
            quantity: "100".to_string(),
            filled_quantity: "100".to_string(),
            status: "Zrealizowane".to_string(),
            currency: "USD".to_string(),
            order_date: "01.04.2025 15:09:10".to_string(),
        }
    }
}
