use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::de::{self, Deserializer};

pub(super) const CSV_HEADER_FIELDS: &[&str] = &[
    "stan",
    "papier",
    "giełda",
    "k/s",
    "liczba zlecona",
    "liczba zrealizowana",
    "limit ceny",
    "walute",
    "limit aktywacji",
    "data zlecenia",
];

fn parse_naive_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(s, "%d.%m-%Y %H:%M:%S").map_err(de::Error::custom)
}

#[derive(Debug, Deserialize)]
pub(super) struct Csv {
    #[serde(rename = "Stan")]
    pub status: String,
    #[serde(rename = "Papier")]
    pub instrument_symbol: String,
    #[serde(rename = "Giełda")]
    pub exchange: String,
    #[serde(rename = "K/S")]
    pub side: String,
    #[serde(rename = "Liczba zlecona")]
    pub quantity: u32,
    #[serde(rename = "Liczba zrealizowana")]
    pub filled_quantity: u32,
    #[serde(rename = "Limit ceny")]
    pub price_limit: String,
    #[serde(rename = "Walute")]
    pub currency: String,
    #[serde(rename = "Limit aktywacji")]
    pub activation_limit: String,
    #[serde(deserialize_with = "parse_naive_datetime", rename = "Data zlecenia")]
    pub order_date: NaiveDateTime,
}
