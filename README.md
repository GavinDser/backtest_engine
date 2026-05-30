# Research Engine

A small Rust backtesting engine for historical market data.

This project is a learning-oriented quantitative research engine. It reads OHLCV price data from CSV, calculates moving-average indicators, generates trading signals, simulates a simple portfolio, and computes basic performance metrics.

The goal is not to predict stock prices or provide trading advice. The goal is to build the core pieces of a backtesting workflow from scratch in Rust.

## Features

- CSV ingestion for OHLCV market data
- `PriceBar` data model for historical price rows
- Close price extraction
- Simple Moving Average calculation
- Buy / Sell / Hold signal generation
- SMA crossover strategy logic
- Simple portfolio simulation with cash and shares
- Equity curve generation
- Total return calculation
- Maximum drawdown calculation
- Unit tests for core logic
- Integration test for the full CSV-to-metrics pipeline

## Implemented From Scratch

The core backtesting logic is implemented manually in Rust:

- SMA calculation
- Signal generation
- Portfolio accounting
- Backtest loop
- Equity curve construction
- Total return
- Maximum drawdown

External crates are used only for infrastructure:

- `csv` for CSV parsing
- `serde` for deserializing CSV rows into Rust structs

## Project Structure

```text
src/
├── main.rs         # Demo entrypoint
├── lib.rs          # Library module exports
├── data.rs         # PriceBar model and CSV loading
├── indicators.rs   # SMA calculation
├── signal.rs       # Buy / Sell / Hold enum
├── strategy.rs     # SMA crossover signal generation
├── portfolio.rs    # Cash, shares, and equity accounting
├── backtest.rs     # Backtest event loop
└── metrics.rs      # Total return and max drawdown

data/
└── sample_prices.csv

tests/
└── full_pipeline_test.rs
```

## Quick Start

Run the demo:

```bash
cargo run
```

Run the test suite:

```bash
cargo test
```

Run lint checks:

```bash
cargo clippy
```

Format the code:

```bash
cargo fmt
```

## Example Demo Output

Using the included sample data, the current demo runs a simple SMA crossover workflow:

```text
CSV -> PriceBar -> close prices -> SMA -> signals -> backtest -> metrics
```

Current sample result:

```text
Total return: ~6.32%
Max drawdown: ~-4.55%
```

These results come from deterministic sample data in `data/sample_prices.csv`. They are for software validation and demonstration only.

## Testing

The project currently includes:

- Unit tests for SMA calculation
- Unit tests for signal generation
- Unit tests for portfolio accounting
- Unit tests for total return and maximum drawdown
- Unit tests for backtest length validation and account simulation
- Integration test for the full CSV-to-metrics pipeline

Current test command:

```bash
cargo test
```

## Design Notes

The project separates the backtesting workflow into small modules:

- `data` handles input data
- `indicators` calculates derived price series
- `strategy` converts indicators into trading signals
- `portfolio` tracks account state
- `backtest` applies signals over time
- `metrics` evaluates performance

This keeps the core logic testable and makes it easier to extend the engine with more indicators, strategies, and cost models.

## Current Limitations

This is still a minimal research engine. It does not yet support:

- Transaction costs
- Slippage
- Multiple assets
- Position sizing
- Short selling
- Benchmark comparison
- Real market data download
- Python bindings
- Visualization

## Roadmap

Planned improvements:

- Add commission and slippage models
- Add RSI indicator
- Add a structured `BacktestResult`
- Add more integration tests
- Add benchmark tests with Criterion
- Add multi-symbol support
- Add Python bindings with PyO3 and maturin
