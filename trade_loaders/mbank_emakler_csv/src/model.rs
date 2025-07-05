use serde::Deserialize;
use chrono::NaiveDateTime;
use serde::de::{self, Deserializer};

fn parse_naive_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(s, "%d.%m-%Y %H:%M:%S")
        .map_err(de::Error::custom)
}

const EXPECTED_CSV_HEADER_FIELDS: &[&str] = &["Stan", "Papier", "Giełda", "K/S", "Liczba zlecona", "Liczba zrealizowana", "Limit ceny", "Walute", "Limit aktywacji", "Data zlecenia"];

#[derive(Debug, Deserialize)]
pub(crate) struct Csv {

    pub stan: String,
    pub papier: String,
    pub gielda: String,
    pub ks: String,
    pub liczba_zlecona: String,
    pub liczba_zrealizowana: String,
    pub limit_ceny: String,
    pub walute: String,
    pub limit_aktywacji: String,
    #[serde(deserialize_with = "parse_naive_datetime")]
    pub data_zlecenia: NaiveDateTime,
}
