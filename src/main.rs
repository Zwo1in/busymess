use anyhow::{Context, Result};
use chrono::NaiveDate;
use reqwest::get;
use serde_json::Value as Json;

const NBP_RATES_URI: &str = "http://api.nbp.pl/api/exchangerates/rates";
const CURRENCY_CODE: &str = "EUR";
const TABLE_NAME: &str = "A";

fn previous_day<DATE: Into<NaiveDate>>(date: DATE) -> NaiveDate {
    date.into() - chrono::Duration::days(1)
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut rate_date = chrono::Utc::now().date_naive();
    let last_euro_rate: Json = loop {
        rate_date = previous_day(rate_date);
        let url = format!("{NBP_RATES_URI}/{TABLE_NAME}/{CURRENCY_CODE}/{rate_date}/");

        if let Ok(json) = get(&url).await?.json().await {
            break json;
        }
    };
    let table = last_euro_rate["rates"][0]["no"]
        .as_str()
        .context(format!("Parsing table from {last_euro_rate}"))?;
    let day: NaiveDate = last_euro_rate["rates"][0]["effectiveDate"]
        .as_str()
        .context(format!("Getting effective date from {last_euro_rate}"))?
        .parse()
        .context(format!("Parsing effective date from {last_euro_rate}"))?;
    let rate = last_euro_rate["rates"][0]["mid"]
        .as_f64()
        .context(format!("Getting rate from {last_euro_rate}"))?;

    println!("{day} - {table}: {rate}");
    Ok(())
}
