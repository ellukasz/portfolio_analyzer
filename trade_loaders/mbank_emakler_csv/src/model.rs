use serde::Deserialize;

pub static HEADER: &str = "Stan;Papier;Giełda;K/S;Liczba zlecona;Liczba zrealizowana;Limit ceny;Walute;Limit aktywacji;Data zlecenia";

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
    pub quantity: String,
    #[serde(rename = "Liczba zrealizowana")]
    pub filled_quantity: String,
    #[serde(rename = "Limit ceny")]
    pub price_limit: String,
    #[serde(rename = "Walute")]
    pub currency: String,
    #[serde(rename = "Limit aktywacji")]
    pub activation_limit: String,
    #[serde(rename = "Data zlecenia")]
    pub order_date: String,
}
