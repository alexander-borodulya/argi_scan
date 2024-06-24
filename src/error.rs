use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArbiError {

    #[error("Failed to parse string as f64: {0}")]
    ParseF64Error(String),

    #[error("Failed to parse string as BigDecimal: {0}")]
    ParseBigDecimalError(String),

    #[error("HTTP request failed: {0}")]
    HttpRequestError(#[from] reqwest::Error),

    #[error("JSON deserialization failed: {0}")]
    JsonDeserializationError(String),

    #[error("Failed to parse string as Decimal: {0}")]
    ParseDecimalError(#[from] rust_decimal::Error),

    #[error("Rate not found for key: {0}")]
    RateNotFoundError(String),
    
    #[error("General error: {0}")]
    GeneralError(#[from] anyhow::Error),
}
