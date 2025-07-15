use super::model::Csv;
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Europe::Warsaw;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use shared_contracts::errors::TradeLoaderError;
use shared_contracts::models::trade_order::{
    InstrumentType, OrderSide, OrderStatus, OrderType, TradeOrder,
};
use std::str::FromStr;

pub(super) fn map(record: Csv) -> Result<TradeOrder, TradeLoaderError> {
    let order = TradeOrder {
        instrument_symbol: record.instrument_symbol.clone(),
        instrument_type: InstrumentType::Stock,
        order_type: _map_order_type(&record)?,

        side: _map_side(&record.side)?,

        quantity: record.fulfilled_quantity.clone(),
        price: _map_price(&record)?,
        commission: _map_commision(&record)?,

        status: match record.status.clone().trim().to_lowercase().as_str() {
            "przyjęte" => OrderStatus::Pending,
            "zamknięte" => OrderStatus::Filled,
            "zrealizowane" => OrderStatus::Filled, // ? how to handle this?
            "anulowane" => OrderStatus::Cancelled,
            "odrzucone" => OrderStatus::Rejected,
            _ => {
                return Err(TradeLoaderError::Parse("Unknown order status".to_string()));
            }
        },
        submission_time: _map_warsaw_time_to_utc(&record.order_date)?,
        currency: record.currency,
        exchange: record.exchange,
    };
    return Ok(order);
}

fn _map_warsaw_time_to_utc(
    date: &chrono::NaiveDateTime,
) -> Result<DateTime<Utc>, TradeLoaderError> {
    let warsaw_dt = match Warsaw.from_local_datetime(&date) {
        chrono::offset::LocalResult::Single(dt) => Ok(dt.with_timezone(&Utc)),
        chrono::offset::LocalResult::Ambiguous(dt1, dt2) => Err(TradeLoaderError::Parse(
            format!("Unknown date time: {}", date).to_string(),
        )),
        chrono::offset::LocalResult::None => Err(TradeLoaderError::Parse(
            format!("Unknown date time: {}", date).to_string(),
        )),
    };
    return warsaw_dt;
}

fn _map_side(side: &str) -> Result<OrderSide, TradeLoaderError> {
    return match side.trim().to_uppercase().as_str() {
        "B" => Ok(OrderSide::Buy),
        "S" => Ok(OrderSide::Sell),
        _ => Err(TradeLoaderError::Parse(
            format!("Unknown order side, side:{}", side).to_string(),
        )),
    };
}

fn _map_order_type(record: &Csv) -> Result<OrderType, TradeLoaderError> {
    if !record.activation_limit.is_empty() {
        return Ok(OrderType::StopLimit);
    } else if !record.price_limit.is_empty() {
        return Ok(OrderType::Limit);
    } else {
        return Err(TradeLoaderError::Parse(
            format!("Unknown order type, record:{:?}", record).to_string(),
        ));
    }
}

fn _map_price(record: &Csv) -> Result<Option<rust_decimal::Decimal>, TradeLoaderError> {
    if !record.price_limit.is_empty() {
        return Decimal::from_str(&record.price_limit)
            .map_err(|e| {
                TradeLoaderError::Parse(format!(
                    "Failed to parse price limit: {}, err: {}",
                    record.price_limit, e
                ))
            })
            .map(|price| Some(price));
    } else {
        return Err(TradeLoaderError::Parse(
            format!("Unknown price, record:{:?}", record).to_string(),
        ));
    }
}

fn _map_commision(record: &Csv) -> Result<Decimal, TradeLoaderError> {
    let fulfilled_quantity: Decimal = Decimal::from_u32(record.fulfilled_quantity)
        .expect("Fulfilled quantity should be a valid u32");

    let price_limit: Decimal = Decimal::from_str(&record.price_limit).map_err(|e| {
        TradeLoaderError::Parse(format!(
            "Failed to parse price limit: {}, e:{}",
            record.price_limit, e
        ))
    })?;

    let mbank_percentage: Decimal = Decimal::from_str("0.039").unwrap();
    let mbank_thrashold = Decimal::from_str("5.00").unwrap();

    let commission = (price_limit * fulfilled_quantity) * mbank_percentage;

    return if commission < mbank_thrashold {
        Ok(mbank_thrashold)
    } else {
        Ok(commission)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
    use chrono_tz::Tz;
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
        let expected_date_time = Utc.with_ymd_and_hms(2025, 07, 14, 17, 20, 25).unwrap();

        assert_eq!(actual_date_time, expected_date_time);
    }

    fn _record_with_price_limit(price_limit: &str, activation_limit: &str) -> Csv {
        Csv {
            price_limit: price_limit.to_string(),
            activation_limit: activation_limit.to_string(),
            instrument_symbol: "AAPL".to_string(),
            exchange: "NASDAQ".to_string(),
            side: "B".to_string(),
            ordered_quantity: 100,
            fulfilled_quantity: 100,
            status: "Zrealizowane".to_string(),
            currency: "USD".to_string(),
            order_date: NaiveDate::from_ymd_opt(2025, 07, 14)
                .unwrap()
                .and_hms_opt(19, 20, 25)
                .unwrap(),
        }
    }

    fn _default_record() -> Csv {
        Csv {
            price_limit: "10.00".to_string(),
            activation_limit: "".to_string(),
            instrument_symbol: "AAPL".to_string(),
            exchange: "NASDAQ".to_string(),
            side: "B".to_string(),
            ordered_quantity: 100,
            fulfilled_quantity: 100,
            status: "Zrealizowane".to_string(),
            currency: "USD".to_string(),
            order_date: NaiveDate::from_ymd_opt(2025, 07, 14)
                .unwrap()
                .and_hms_opt(19, 20, 25)
                .unwrap(),
        }
    }
}
