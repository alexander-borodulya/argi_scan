use std::collections::{BTreeMap, HashSet};
use bigdecimal::{BigDecimal, FromPrimitive, Zero};
use std::str::FromStr;
use anyhow::{Result, Context};

use crate::{config::AppConfig, error::ArbiError};

pub fn build_graph_btm(rates: &BTreeMap<String, BigDecimal>) -> Result<BTreeMap<String, BTreeMap<String, BigDecimal>>, ArbiError> {
    let mut graph = BTreeMap::new();

    for (pair, rate) in rates {
        let currencies: Vec<&str> = pair.split('-').collect();
        let from = currencies[0].to_string();
        let to = currencies[1].to_string();

        let rate_decimal = rust_decimal::Decimal::from_str(&rate.to_string())
            .context("Failed to parse rate as Decimal")?;

        let ln_rate = rust_decimal::MathematicalOps::ln(&rate_decimal);
        let ln_rate_str = ln_rate.to_string();
        let ln_rate_bd = BigDecimal::from_str(&ln_rate_str)
            .context("Failed to parse ln_rate as BigDecimal")?;

        graph
            .entry(from.clone())
            .or_insert_with(BTreeMap::new)
            .insert(to.clone(), -ln_rate_bd);
    }

    Ok(graph)
}

pub fn find_arbitrage_with_base_currency_btm(base_currency: String, graph: &BTreeMap<String, BTreeMap<String, BigDecimal>>) -> Option<Vec<Vec<String>>> {
    let vertices: Vec<String> = graph.keys().cloned().collect();
    let mut distances = BTreeMap::new();
    let mut predecessors = BTreeMap::new();

    for vertex in &vertices {
        distances.insert(vertex.clone(), BigDecimal::from_str("1000000000").unwrap());
        predecessors.insert(vertex.clone(), None);
    }

    distances.insert(base_currency.clone(), BigDecimal::zero());

    // Relax edges |V| - 1 times
    for _ in 0..vertices.len() - 1 {
        for u in &vertices {
            if let Some(neighbors) = graph.get(u) {
                for (v, weight) in neighbors {
                    let distance_u = distances[u].clone();
                    let new_distance = &distance_u + weight;

                    if new_distance < distances[v] {
                        distances.insert(v.clone(), new_distance);
                        predecessors.insert(v.clone(), Some(u.clone()));
                    }
                }
            }
        }
    }

    // Check for negative-weight cycles and reconstruct paths
    let mut arbitrages = vec![];

    for u in &vertices {
        if let Some(neighbors) = graph.get(u) {
            for (v, weight) in neighbors {
                let distance_u = distances[u].clone();
                let new_distance = &distance_u + weight;

                if new_distance < distances[v] {
                    // Negative cycle detected
                    let mut cycle = vec![v.clone()];
                    let mut current = u.clone();
                    let mut visited = HashSet::new();
                    
                    while !visited.contains(&current) {
                        visited.insert(current.clone());
                        cycle.push(current.clone());
                        current.clone_from(predecessors[&current].as_ref().unwrap());
                    }

                    cycle.push(current.clone());
                    cycle.reverse();

                    arbitrages.push(cycle.clone());
                }
            }
        }
    }

    if !arbitrages.is_empty() {
        return Some(arbitrages);
    }

    None
}

pub fn adjust_arbitrages_btm(base_currency: String, arbitrage: &[String]) -> Vec<String> {
    let mut adjusted_arbitrage = arbitrage.to_owned();

    // Ensure the arbitrage starts with the base currency
    if adjusted_arbitrage.first() != Some(&base_currency) {
        adjusted_arbitrage.insert(0, base_currency.clone());
    }

    // Ensure the arbitrage ends with the base currency
    if adjusted_arbitrage.last() != Some(&base_currency) {
        adjusted_arbitrage.push(base_currency.clone());
    }

    // Remove redundant conversions
    adjusted_arbitrage = remove_consecutive_duplicates(&adjusted_arbitrage);

    adjusted_arbitrage
}

pub fn remove_consecutive_duplicates(input: &[String]) -> Vec<String> {
    if input.is_empty() {
        return vec![];
    }
    // TODO: This could be done more efficiently
    input.to_owned()
}

pub fn calculate_arbitrage_btm(investment: BigDecimal, arbitrage: &[String], rates: &BTreeMap<String, BigDecimal>) -> Result<BigDecimal, ArbiError> {
    let mut result = investment;

    for i in 0..arbitrage.len() - 1 {
        let from = &arbitrage[i];
        let to = &arbitrage[i + 1];
        let key = format!("{}-{}", from, to);
        let rate = rates.get(&key).ok_or_else(|| ArbiError::RateNotFoundError(key.clone()))?;
        result *= rate;
    }

    Ok(result)
}

pub fn best_pnl(arbitrages: Vec<(Vec<String>, BigDecimal)>) -> Option<(Vec<String>, BigDecimal)> {
    arbitrages.into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
}

pub fn process_rates_btm(config: &AppConfig, rates: &BTreeMap<String, BigDecimal>) -> Result<Option<(Vec<String>, BigDecimal)>, ArbiError> {
    // println!("rates: {:#?}", rates);
    let graph = build_graph_btm(rates)?;
    // println!("graph: {:#?}", graph);
    
    let mut pnls = Vec::new();


    if let Some(arbitrages) = find_arbitrage_with_base_currency_btm(config.base_currency.clone(), &graph) {
        let investment = 100.0;
        let investment = BigDecimal::from_f64(investment).context("Failed to convert investment to BigDecimal")?;
        println!("Possible arbitrages: {}", arbitrages.len());
        for (i, arbitrage) in arbitrages.iter().enumerate() {
            let arbitrage = adjust_arbitrages_btm(config.base_currency.clone(), arbitrage);
            let arbitrage_result = calculate_arbitrage_btm(investment.clone(), &arbitrage, rates)?;
            println!("{}, path: {:?}, result: {}", i, arbitrage, arbitrage_result);
            pnls.push((arbitrage, arbitrage_result));
        }
    } else {
        println!("No arbitrage found");
    }

    Ok(best_pnl(pnls))
}

pub fn run_with_default_rates_btm(config: &AppConfig) -> Result<Option<(Vec<String>, BigDecimal)>, ArbiError> {
    let rates = crate::http_rates::default_rates_bigdecimal()?
        .into_iter()
        .collect();
    process_rates_btm(config, &rates)
}

pub fn run_with_fetch_rates_btm(config: &AppConfig) -> Result<Option<(Vec<String>, BigDecimal)>, ArbiError> {
    let rates = crate::http_rates::fetch_rates()?
        .into_iter()
        .collect();
    process_rates_btm(config, &rates)
}
