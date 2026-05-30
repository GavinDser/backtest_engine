use research_engine::backtest::run_backtest;
use research_engine::data::{close_prices, load_price_bars};
use research_engine::indicators::simple_moving_average;
use research_engine::metrics::{max_drawdown, total_return};
use research_engine::strategy::generate_sma_signals;

#[test]
fn test_full_pipeline_from_csv_to_metrics() {
    let path = "./data/sample_prices.csv";

    let bars = load_price_bars(path).expect("path not usable");
    let prices = close_prices(&bars);

    let short_sma = simple_moving_average(&prices, 2);
    let long_sma = simple_moving_average(&prices, 3);

    let signals = generate_sma_signals(&short_sma, &long_sma).expect("SMA signal generation wrong");

    let equity_curve = run_backtest(&prices, &signals, 10000.0).expect("length is wrong");

    let ret = total_return(&equity_curve);
    let mdd = max_drawdown(&equity_curve);

    assert!(!bars.is_empty());
    assert_eq!(prices.len(), bars.len());
    assert_eq!(short_sma.len(), prices.len());
    assert_eq!(long_sma.len(), prices.len());
    assert_eq!(signals.len(), prices.len());
    assert_eq!(equity_curve.len(), prices.len());
    assert!(ret.is_some());
    assert!(mdd.is_some());
}