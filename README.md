# Arbitrage Scanner

This is a Rust project that scans for arbitrage opportunities across multiple currency exchange rates using the Bellman-Ford algorithm to find negative-weight cycles in a graph representation, which indicates the presence of an arbitrage opportunity.

## Implementation Details

- Builds a graph data structure from a set of currency exchange rates
- Implements the Bellman-Ford algorithm to detect negative-weight cycles (arbitrage opportunities)
- Supports both default rates and fetching rates from an external source
- Uses the `bigdecimal` crate for precise decimal arithmetic (over `f64` base type)
- Calculates the potential profit (or loss) from an arbitrage opportunity

## Algorithmic Complexity Analysis

The core algorithm time complexity is O(|V| * |E|), where |V| is the number of vertices (currencies) and |E| is the number of edges (exchange rates) in the graph.

## CLI Usage

```
Usage: arbi_scan [OPTIONS]

Options:
  -b, --base-currency <BASE_CURRENCY>
          The base currency for arbitrage calculations [default: EUR]
  -i, --initial-investment <INITIAL_INVESTMENT>
          The initial investment amount [default: 100]
  -r, --run-mode <RUN_MODE>
          Run mode: Demo or Fetch [default: demo]
  -h, --help
          Print help
```
