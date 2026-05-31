use research_engine::backtest::run_backtest_summary;
use research_engine::data::{close_prices, load_price_bars};
use research_engine::indicators::simple_moving_average;
use research_engine::strategy::generate_sma_signals;

use research_engine::config::BacktestConfig;

#[test]
fn test_full_pipeline_from_csv_to_metrics() {
    let path = "./data/sample_prices.csv";

    let bars = load_price_bars(path).expect("path not usable");
    let prices = close_prices(&bars);

    let short_sma = simple_moving_average(&prices, 2);
    let long_sma = simple_moving_average(&prices, 3);

    let signals = generate_sma_signals(&short_sma, &long_sma).expect("SMA signal generation wrong");

    let initial_cash = 10000.0;

    let backtest_config = BacktestConfig {
        initial_cash,
        commission_rate: 0.004,
    };

    let result = run_backtest_summary(&prices, &signals, &backtest_config)
        .expect("Backtest result should not be empty");

    assert!(!bars.is_empty());
    assert_eq!(prices.len(), bars.len());
    assert_eq!(short_sma.len(), prices.len());
    assert_eq!(long_sma.len(), prices.len());
    assert_eq!(signals.len(), prices.len());
    assert_eq!(result.equity_curve.len(), prices.len());
    assert!(result.total_return.is_some());
    assert!(result.max_drawdown.is_some());
}
