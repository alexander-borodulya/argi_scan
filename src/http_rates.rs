use std::collections::HashMap;
use bigdecimal::BigDecimal;
use serde::Deserialize;
use std::str::FromStr;
use anyhow::{Result, Context};

use crate::error::ArbiError;

const DEFAULT_RATES_URL: &str = "https://api.swissborg.io/v1/challenge/rates";

#[derive(Deserialize, Debug)]
pub struct RateResponse {
    rates: HashMap<String, String>,
}

impl RateResponse {
    #[allow(dead_code)]
    pub fn parsed_as_f64<K, V>(rates: &HashMap<K, V>) -> Result<HashMap<String, f64>, ArbiError>
    where
        K: Into<String> + Clone,
        V: AsRef<str>,
    {
        let mut rates_map = HashMap::new();
        for (key, value) in rates {
            let value = f64::from_str(value.as_ref())
                .map_err(|_| ArbiError::ParseF64Error(value.as_ref().to_string()))?;
            rates_map.insert(key.clone().into(), value);
        }
        Ok(rates_map)
    }

    /// Parse the rates as BigDecimal to provide better precision comparing to f64
    pub fn parsed_as_bigdecimal<K, V>(rates: &HashMap<K, V>) -> Result<HashMap<String, BigDecimal>, ArbiError>
    where
        K: Into<String> + Clone,
        V: AsRef<str>,
    {
        let mut rates_map = HashMap::new();
        for (key, value) in rates {
            let value = BigDecimal::from_str(value.as_ref())
                .map_err(|_| ArbiError::ParseBigDecimalError(value.as_ref().to_string()))?;
            rates_map.insert(key.clone().into(), value);
        }
        Ok(rates_map)
    }

    pub fn cloned_as_f64(&self) -> Result<HashMap<String, f64>, ArbiError> {
        let mut rates_map = HashMap::new();
        for (key, value) in self.rates.iter() {
            let value = f64::from_str(value)
                .map_err(|_| ArbiError::ParseF64Error(value.to_string()))?;
            rates_map.insert(key.clone(), value);
        }
        Ok(rates_map)
    }
}

/// Fetch the exchange rates from the API
pub fn fetch_rates() -> Result<HashMap<String, BigDecimal>, ArbiError> {
    let url = DEFAULT_RATES_URL;
    let response = reqwest::blocking::get(url)
        .context("Failed to send request to API")?;
    let rates_response: RateResponse = response.json()
        .context("Failed to deserialize response as RateResponse")?;
    RateResponse::parsed_as_bigdecimal(&rates_response.rates)
}

// EUR DAI BTC EUR
// 100 * 5.0427577751 * 0.2053990550 * 0.0000429088 * 23258.8865583847
// 103,3717428183

pub fn default_rates_bigdecimal() -> Result<HashMap<String, BigDecimal>, ArbiError> {
    let rates = HashMap::from([
        ("BTC-BTC", "1.0000000000"),
        ("BTC-BORG", "116352.2654440156"),
        ("BTC-DAI", "23524.1391553039"),
        ("BTC-EUR", "23258.8865583847"),
        ("BORG-BTC", "0.0000086866"),
        ("BORG-BORG", "1.0000000000"),
        ("BORG-DAI", "0.2053990550"),
        ("BORG-EUR", "0.2017539914"),
        ("DAI-BTC", "0.0000429088"),
        ("DAI-BORG", "4.9320433378"),
        ("DAI-DAI", "1.0000000000"),
        ("DAI-EUR", "0.9907652193"),
        ("EUR-BTC", "0.0000435564"),
        ("EUR-BORG", "5.0427577751"),
        ("EUR-DAI", "1.0211378960"),
        ("EUR-EUR", "1.0000000000"),
    ]);
    RateResponse::parsed_as_bigdecimal(&rates)
}
