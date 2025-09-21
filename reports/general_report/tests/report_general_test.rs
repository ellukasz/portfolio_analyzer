use chrono::TimeZone;
use chrono::Utc;
use general_report::report;
use shared_contracts::models::money::Money;
mod test_data;

#[test]
fn trade_period_calculation() {
    let report = report::create(test_data::orders()).unwrap();
    assert_eq!(
        report.trade_period.start,
        Utc.with_ymd_and_hms(2023, 1, 15, 10, 0, 0).unwrap()
    );
    assert_eq!(
        report.trade_period.end,
        Utc.with_ymd_and_hms(2025, 2, 15, 10, 0, 0).unwrap()
    );
}

#[test]
fn buy_total_quantity() {
    let report = report::create(test_data::orders()).unwrap();
    assert_eq!(report.buy_summary.total_quantity, 3);
}

#[test]
fn buy_total_value() {
    let report = report::create(test_data::orders()).unwrap();
    assert_eq!(report.buy_summary.total_value, Money::from_i128(3000));
}
#[test]
fn buy_total_commission() {
    let report = report::create(test_data::orders()).unwrap();
    assert_eq!(report.buy_summary.total_commission, Money::from_i128(300));
}
