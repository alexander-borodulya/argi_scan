#[derive(Debug, Clone)]
pub struct AppConfig {
    pub initial_investment: f64,
    pub base_currency: String,
}

impl AppConfig {
    pub fn new(initial_investment: f64, base_currency: String) -> Self {
        AppConfig {
            initial_investment,
            base_currency,
        }
    }
}
