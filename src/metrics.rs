pub fn total_return(equity_curve: &[f64]) -> Option<f64> {
    if equity_curve.is_empty() {
        return None;
    }

    Some((equity_curve[equity_curve.len() - 1] - equity_curve[0]) / equity_curve[0])
}

pub fn max_drawdown(equity_curve: &[f64]) -> Option<f64> {
    if equity_curve.is_empty() {
        return None;
    }

    let mut peak = equity_curve[0];
    let mut max_drawdown = 0.0;

    for &item in equity_curve {
        if item > peak {
            peak = item;
        }
        let drawdown = (item - peak) / peak;

        if drawdown < max_drawdown {
            max_drawdown = drawdown;
        }
    }

    Some(max_drawdown)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 1e-9);
    }

    #[test]
    fn test_total_return() {
        let equity_curve = [100.0, 110.0, 120.0];
        let result = total_return(&equity_curve).unwrap();
        assert_close(result, 0.2);
    }

    #[test]
    fn test_total_return_empty_curve() {
        let equity_curve: Vec<f64> = Vec::new();
        assert_eq!(total_return(&equity_curve), None);
    }

    #[test]
    fn test_max_drawdown() {
        let equity_curve = [100.0, 120.0, 90.0, 130.0];
        let result = max_drawdown(&equity_curve).unwrap();
        assert_close(result, -0.25);
    }

    #[test]
    fn test_max_drawdown_empty_curve() {
        let equity_curve: Vec<f64> = Vec::new();
        assert_eq!(max_drawdown(&equity_curve), None);
    }

    #[test]
    fn test_max_drawdown_monotonic_up() {
        let equity_curve = [100.0, 110.0, 120.0];
        let result = max_drawdown(&equity_curve).unwrap();
        assert_close(result, 0.0);
    }
}
