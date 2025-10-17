use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

const DEFAULT_MONEY_SCALE: u32 = 2;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Money(pub Decimal);

impl Money {
    pub fn zero() -> Self {
        Money(Decimal::new(0, DEFAULT_MONEY_SCALE))
    }

    pub fn from_i128(value: i128) -> Self {
        let val = Decimal::from_i128_with_scale(value, DEFAULT_MONEY_SCALE);
        Money(val)
    }

    pub fn from_string(value: &str) -> Self {
        //TODO fix error handling
        if value.trim().is_empty() {
            return Money::zero();
        }

        let value_normalized = value.replace(',', ".");
        let mut val =
            Decimal::from_str(&value_normalized).expect("Failed to convert str to Decimal");
        val.rescale(DEFAULT_MONEY_SCALE);
        Money(val)
    }

    pub fn from_decimal(value: Decimal) -> Self {
        let mut v = value;
        v.rescale(DEFAULT_MONEY_SCALE);
        Money(v)
    }

    pub fn as_decimal(&self) -> Decimal {
        self.0
    }

    pub fn as_i128(&self) -> i128 {
        self.0.mantissa()
    }

    pub fn as_string(&self) -> String {
        self.0.to_string()
    }
}
