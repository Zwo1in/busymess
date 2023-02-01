use std::fmt::Display;

use askama::Template;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "main.tex", syntax = "tex", escape = "none")]
#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    invoice: InvoiceInfo,
    invoicer: CompanyInfo,
    invoicee: CompanyInfo,
    account: PaymentInfo,
    product: Vec<Product>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct InvoiceInfo {
    id: String,
    date: String,
    due_date: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct CompanyInfo {
    name: String,
    vat_id: String,
    address: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct Product {
    description: String,
    quantity: f64,
    rate: f64,
    tax: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct PaymentInfo {
    iban: String,
    bank_code: BankCode,
    bank_name: String,
    address: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "UPPERCASE")]
enum BankCode {
    Swift(String),
    Bic(String),
}

impl Display for BankCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BankCode::Bic(ref code) => write!(f, "BIC: {code}"),
            BankCode::Swift(ref code) => write!(f, "SWIFT: {code}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use figment::{
        providers::{Format, Toml},
        Figment,
    };

    #[test]
    fn config_test() {
        figment::Jail::expect_with(|jail| {
            jail.create_file(
                "busymess.toml",
                r#"
        [invoice]
        id = "2023/01/31"
        date = "31.01.2023"
        due_date = "14.02.2023"

        [invoicer]
        name = "Alan Moore"
        address = "ul. Alpejska 3A, 50-552 Wroclaw, Poland"
        vat_id = "PL1234567890"

        [invoicee]
        name = "Elvis Presley"
        address = "ul. Sudecka 1C, 50-232 Wroclaw, Poland"
        vat_id = "PL0987654321"

        [account]
        iban = "PL1111222233334444"
        bank_code = { type = "SWIFT", value = "ACKXXDXX" }
        bank_name = "PDK"
        address = "ul. Pirenejska 2B, 50-343 Wroclaw, Poland"

        [[product]]
        description = "Software development"
        quantity = 1
        rate = 5000
        tax = 0

        [[product]]
        description = "Consulting"
        quantity = 1
        rate = 500
        tax = 0
        "#,
            )?;

            let figment = Figment::new().merge(Toml::file("busymess.toml"));

            let config: Config = figment.extract()?;
            assert_eq!(
                config,
                Config {
                    invoice: InvoiceInfo {
                        id: "2023/01/31".to_owned(),
                        date: "31.01.2023".to_owned(),
                        due_date: "14.02.2023".to_owned()
                    },
                    invoicer: CompanyInfo {
                        name: "Alan Moore".to_owned(),
                        address: "ul. Alpejska 3A, 50-552 Wroclaw, Poland".to_owned(),
                        vat_id: "PL1234567890".to_owned(),
                    },
                    invoicee: CompanyInfo {
                        name: "Elvis Presley".to_owned(),
                        address: "ul. Sudecka 1C, 50-232 Wroclaw, Poland".to_owned(),
                        vat_id: "PL0987654321".to_owned(),
                    },
                    account: PaymentInfo {
                        iban: "PL1111222233334444".to_owned(),
                        bank_code: BankCode::Swift("ACKXXDXX".to_owned()),
                        bank_name: "PDK".to_owned(),
                        address: "ul. Pirenejska 2B, 50-343 Wroclaw, Poland".to_owned(),
                    },
                    product: vec![
                        Product {
                            description: "Software development".to_owned(),
                            quantity: 1.,
                            rate: 5000.,
                            tax: 0.,
                        },
                        Product {
                            description: "Consulting".to_owned(),
                            quantity: 1.,
                            rate: 500.,
                            tax: 0.,
                        }
                    ],
                }
            );

            Ok(())
        });
    }
}
