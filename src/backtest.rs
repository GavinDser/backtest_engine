use crate::signal::Signal;

use crate::portfolio::Portfolio;

pub fn run_backtest(
    prices: &[f64],
    signals: &[Signal],
    initial_cash: f64,
) -> Result<Vec<f64>, BacktestError> {
    if prices.len() != signals.len() {
        return Err(BacktestError::LengthMismatch {
            price_len: prices.len(),
            signal_len: signals.len(),
        });
    }

    let mut backtest_result: Vec<f64> = Vec::new();
    let mut portfolio = Portfolio::new(initial_cash);

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

        let result = run_backtest(&prices, &signals, 1000.0).unwrap();

        assert_eq!(result.len(), 3);
        assert_close(result[0], 1000.0);
        assert_close(result[1], 1200.0);
        assert_close(result[2], 1100.0);
    }

    #[test]
    fn test_run_backtest_length_mismatch() {
        let prices = vec![10.0, 12.0];
        let signals = vec![Signal::Buy];

        let result = run_backtest(&prices, &signals, 1000.0);

        assert_eq!(
            result,
            Err(BacktestError::LengthMismatch {
                price_len: 2,
                signal_len: 1,
            })
        );
    }
}
