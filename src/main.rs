use anyhow::{Result, Context};
use arbi_scan::algo_bigdecimal_btm;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    /// The base currency for arbitrage calculations
    #[arg(short, long, default_value = "EUR")]
    base_currency: String,

    /// The initial investment amount
    #[arg(short, long, default_value_t = 100.0)]
    initial_investment: f64,

    /// Run mode: Demo or Fetch
    #[arg(short, long, default_value = "fetch")]
    run_mode: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let config = arbi_scan::config::AppConfig::new(args.initial_investment, args.base_currency);

    if let Some((arbitrage, pnl)) = match args.run_mode.as_str() {
        "fetch" => algo_bigdecimal_btm::run_with_fetch_rates_btm(&config).context("Run failed in fetch mode")?,
        "demo" => algo_bigdecimal_btm::run_with_default_rates_btm(&config).context("Run failed in demo mode")?,
        _ => panic!("Unknown run mode: {}", args.run_mode),
    } {
        println!("Initial_investment: {}, Arbitrage: {:?}, PnL: {}", config.initial_investment, arbitrage, pnl);
    } else {
        println!("No arbitrage found");
    }

    Ok(())
}
