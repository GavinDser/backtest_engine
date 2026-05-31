use crate::signal::Signal;

use crate::portfolio::Portfolio;

use crate::metrics::{max_drawdown, total_return};

use crate::config::BacktestConfig;

#[derive(Debug, Clone)]
pub struct BacktestResult {
    pub equity_curve: Vec<f64>,
    pub initial_equity: f64,
    pub final_equity: f64,
    pub total_return: Option<f64>,
    pub max_drawdown: Option<f64>,
}

pub fn run_backtest(
    prices: &[f64],
    signals: &[Signal],
    config: &BacktestConfig,
) -> Result<Vec<f64>, BacktestError> {
    if prices.len() != signals.len() {
        return Err(BacktestError::LengthMismatch {
            price_len: prices.len(),
            signal_len: signals.len(),
        });
    }

    let mut backtest_result: Vec<f64> = Vec::new();
    let mut portfolio = Portfolio::new(config.initial_cash);

    for i in 0..prices.len() {
        let price = prices[i];
        let signal = signals[i];

        match signal {
            Signal::Buy => portfolio.buy_all(price),
            Signal::Sell => portfolio.sell_all(price),
            Signal::Hold => {}
        }

        let equity = portfolio.equity(price);
        backtest_result.push(equity);
    }

    Ok(backtest_result)
}

pub fn run_backtest_summary(
    prices: &[f64],
    signals: &[Signal],
    config: &BacktestConfig,
) -> Result<BacktestResult, BacktestError> {
    let equity_curve = run_backtest(prices, signals, config)?;
    let final_equity = equity_curve.last().copied().unwrap_or(config.initial_cash);
    let total_return = total_return(&equity_curve);
    let max_drawdown = max_drawdown(&equity_curve);

    Ok(BacktestResult {
        equity_curve,
        initial_equity: config.initial_cash,
        final_equity,
        total_return,
        max_drawdown,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BacktestError {
    LengthMismatch { price_len: usize, signal_len: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 1e-9);
    }

    #[test]
    fn test_run_backtest_buy_hold_sell() {
        let prices = vec![10.0, 12.0, 11.0];
        let signals = vec![Signal::Buy, Signal::Hold, Signal::Sell];
        let backtest_config = BacktestConfig {
            initial_cash: 1000.0,
            commission_rate: 0.004,
        };

        let result = run_backtest(&prices, &signals, &backtest_config).unwrap();

        assert_eq!(result.len(), 3);
        assert_close(result[0], 1000.0);
        assert_close(result[1], 1200.0);
        assert_close(result[2], 1100.0);
    }

    #[test]
    fn test_run_backtest_length_mismatch() {
        let prices = vec![10.0, 12.0];
        let signals = vec![Signal::Buy];
        let backtest_config = BacktestConfig {
            initial_cash: 1000.0,
            commission_rate: 0.004,
        };

        let result = run_backtest(&prices, &signals, &backtest_config);

        assert_eq!(
            result,
            Err(BacktestError::LengthMismatch {
                price_len: 2,
                signal_len: 1,
            })
        );
    }

    #[test]
    fn test_run_backtest_summary() {
        let prices = [10.0, 12.0, 11.0];
        let signals = [Signal::Buy, Signal::Hold, Signal::Sell];
        let backtest_config = BacktestConfig {
            initial_cash: 1000.0,
            commission_rate: 0.004,
        };

        let result = run_backtest_summary(&prices, &signals, &backtest_config).unwrap();

        assert_eq!(result.equity_curve.len(), 3);
        assert_close(result.final_equity, 1100.0);
        assert_close(result.total_return.unwrap(), 0.1);
        assert_close(result.initial_equity, 1000.0);
        assert_close(result.max_drawdown.unwrap(), -0.0833333333333333333333);
    }
}
