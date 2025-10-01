use chrono::TimeZone;
use chrono::Utc;
use general_report::report;
use general_report::report::Report;
use shared_contracts::models::money::Money;
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
fn instruments_purchase_value() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.purchase_value, 3000);
}

#[test]
fn instruments_sale_value() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.sale_value, 6000);
}

#[test]
fn instruments_cost_basis() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.cost_basis, 3300);
}

#[test]
fn instruments_net_proceeds() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.net_proceeds, 5700);
}

#[test]
fn instruments_average_cost_basis() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.average_cost_basis, 1100);
}

#[test]
fn instruments_tax_base() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.tax_base, 2400);
}

#[test]
fn instruments_tax_amount() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.tax_amount, Money::from_i128(456));
}

fn _create_report() -> Report {
    report::create(test_data::orders()).unwrap()
}
