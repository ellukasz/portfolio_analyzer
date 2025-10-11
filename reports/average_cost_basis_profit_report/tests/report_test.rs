use average_cost_basis_profit_report::report;
use chrono::TimeZone;
use chrono::Utc;
use shared_contracts::models::money::Money;
use shared_contracts::models::report::ProfitReport;
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
        Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap()
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
    assert_eq!(instruments.buy_commission, Money::from_i128(300));
}

#[test]
fn instruments_sell_commission() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.sell_commission, Money::from_i128(300));
}

#[test]
fn instruments_purchase_value() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.purchase_value, Money::from_i128(3000));
}

#[test]
fn instruments_sale_value() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.sale_value, Money::from_i128(6000));
}

#[test]
fn instruments_cost_basis() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.cost_basis, Money::from_i128(3300));
}

#[test]
fn instruments_net_proceeds() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.net_proceeds, Money::from_i128(5700));
}

#[test]
fn instruments_average_cost_basis() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.average_cost_basis, Money::from_i128(1100));
}

#[test]
fn instruments_tax_base() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.tax_base, Money::from_i128(2400));
}

#[test]
fn instruments_tax_amount() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.tax_amount, Money::from_i128(456));
}

// net_proceeds sell_value - sell_commission
// => 6000 - 300 = 5700
// cost_basis = purchase_value + buy_commission
// => 3000 + 300 = 3300
// average_cost_basis = cost_basis / sell_quantity
// => 3300 / 3 = 1100
// tax_base = net_proceeds - (average_cost_basis * sell_quantity)
// => 5700 - (1100 * 3) = 2400
// tax_amount = tax_base * 19%
//=> 2400 * 19% = 456
// net_profit = net_proceeds - cost_basis- tax_amount
// => 5700 - 3300 - 456 = 1944
#[test]
fn instruments_net_profit() {
    let instruments = &_create_report().instruments[0];
    assert_eq!(instruments.net_profit, Money::from_i128(1944));
}

#[test]
fn summary_first_order_at() {
    let summary = &_create_report().summary;
    assert_eq!(
        summary.trade_period.start,
        Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap()
    );
    assert_eq!(
        summary.trade_period.end,
        Utc.with_ymd_and_hms(2026, 1, 2, 10, 0, 0).unwrap()
    );
}

#[test]
fn summary_total_commission() {
    let summary = &_create_report().summary;
    assert_eq!(summary.commission_total, Money::from_i128(600));
}

#[test]
fn summary_tax_amount_total() {
    let summary = &_create_report().summary;
    assert_eq!(summary.tax_amount_total, Money::from_i128(456));
}

#[test]
fn summary_net_profit_total() {
    let summary = &_create_report().summary;
    assert_eq!(summary.net_profit_total, Money::from_i128(1944));
}

fn _create_report() -> ProfitReport {
    report::create(test_data::orders()).unwrap()
}
