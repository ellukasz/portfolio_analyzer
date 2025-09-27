use chrono::TimeZone;
use chrono::Utc;
use general_report::report;
use general_report::report::Report;
mod test_data;

#[test]
fn instruments_instrument_symbol() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.instrument_symbol, "AAPL");
}

#[test]
fn instruments_first_order_at() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(
        instruments.trade_period.start,
        Utc.with_ymd_and_hms(2023, 1, 15, 10, 0, 0).unwrap()
    );
}

#[test]
fn instruments_last_order_at() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(
        instruments.trade_period.end,
        Utc.with_ymd_and_hms(2026, 1, 2, 10, 0, 0).unwrap()
    );
}

#[test]
fn instruments_buy_quantity() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.buy_quantity, 3);
}

#[test]
fn instruments_sell_quantity() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.sell_quantity, 3);
}

#[test]
fn instruments_buy_commission() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.buy_commission, 300);
}

#[test]
fn instruments_sell_commission() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.sell_commission, 300);
}

#[test]
fn instruments_buy_gross_value() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.buy_gross_value, 3000);
}

#[test]
fn instruments_sell_gross_value() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.sell_gross_value, 6000);
}

#[test]
fn instruments_buy_net_value() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.buy_net_value, 3300);
}

#[test]
fn instruments_sell_net_value() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.sell_net_value, 6300);
}

#[test]
fn instruments_buy_average_value() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.buy_average_value, 1100);
}

fn _create_report() -> Report {
    report::create(test_data::orders()).unwrap()
}
