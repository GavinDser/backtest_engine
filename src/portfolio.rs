pub struct Portfolio {
    pub cash: f64,
    pub shares: f64,
}

impl Portfolio {
    pub fn new(initial_cash: f64) -> Self {
        Portfolio {
            cash: initial_cash,
            shares: 0.0,
        }
    }

    pub fn equity(&self, price: f64) -> f64 {
        self.shares * price + self.cash
    }

    pub fn buy_all(&mut self, price: f64) {
        if self.cash > 0.0 {
            self.shares = self.cash / price;
            self.cash = 0.0;
        }
    }

    pub fn sell_all(&mut self, price: f64) {
        if self.shares > 0.0 {
            self.cash = self.shares * price;
            self.shares = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 1e-9);
    }

    #[test]
    fn test_portfolio_new_and_equity() {
        let portfolio = Portfolio::new(1000.0);

        assert_close(portfolio.cash, 1000.0);
        assert_close(portfolio.shares, 0.0);
        assert_close(portfolio.equity(10.0), 1000.0);
    }

    #[test]
    fn test_portfolio_buy_sell_and_equity() {
        let mut portfolio = Portfolio::new(1000.0);

        portfolio.buy_all(10.0);
        assert_close(portfolio.cash, 0.0);
        assert_close(portfolio.shares, 100.0);
        assert_close(portfolio.equity(10.0), 1000.0);
        assert_close(portfolio.equity(12.0), 1200.0);

        portfolio.sell_all(12.0);
        assert_close(portfolio.cash, 1200.0);
        assert_close(portfolio.shares, 0.0);
        assert_close(portfolio.equity(12.0), 1200.0);
    }
}
