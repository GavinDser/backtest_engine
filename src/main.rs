use research_engine::backtest::run_backtest_summary;
use research_engine::config::BacktestConfig;
use research_engine::data::{close_prices, load_price_bars};
use research_engine::indicators::simple_moving_average;
use research_engine::strategy::generate_sma_signals;

fn main() {
    let path = "./data/sample_prices.csv";

    let bars = load_price_bars(path).expect("path not usable");
    let prices = close_prices(&bars);

    let short_sma = simple_moving_average(&prices, 2);
    let long_sma = simple_moving_average(&prices, 3);

    let signals = generate_sma_signals(&short_sma, &long_sma).expect("SMA signal generation wrong");

    let initial_cash = 10000.0;
    let commission_rate = 0.001;

    let backtest_config = BacktestConfig {
        initial_cash,
        commission_rate,
    };

    let result = run_backtest_summary(&prices, &signals, &backtest_config)
        .expect("Backtest result should not be empty");

    println!("Rows loaded: {}", bars.len());
    println!("Initial equity: {:.2}", result.initial_equity);
    println!("Final equity: {:.2}", result.final_equity);
    println!(
        "Total return: {:.2}%",
        result.total_return.unwrap_or(0.0) * 100.0
    );
    println!(
        "Max drawdown: {:.2}%",
        result.max_drawdown.unwrap_or(0.0) * 100.0
    );
}
