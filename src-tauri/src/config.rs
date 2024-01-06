use std::str::FromStr;

use bdk::bitcoin::Network;
use serde::{Deserialize, Serialize};

pub static NETWORK: Network = Network::Regtest;
pub static APP_NAME: &str = "frostydupy";

#[derive(Debug, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CountryCurrency {
    EUR,
    USD,
}

impl FromStr for CountryCurrency {
    type Err = ();
    fn from_str(input: &str) -> Result<CountryCurrency, Self::Err> {
        match input {
            "EUR" => Ok(CountryCurrency::EUR),
            "Eur" => Ok(CountryCurrency::EUR),
            "eur" => Ok(CountryCurrency::EUR),
            "USD" => Ok(CountryCurrency::USD),
            "Usd" => Ok(CountryCurrency::USD),
            "usd" => Ok(CountryCurrency::USD),
            _ => Err(()),
        }
    }
}

impl FromStr for Theme {
    type Err = ();
    fn from_str(input: &str) -> Result<Theme, Self::Err> {
        match input {
            "light" => Ok(Theme::Light),
            "dark" => Ok(Theme::Dark),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub btc_network: Network,
    pub theme: Theme,
    pub fiat_currency: CountryCurrency,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            btc_network: NETWORK,
            theme: Theme::Dark,
            fiat_currency: CountryCurrency::EUR,
        }
    }
}
