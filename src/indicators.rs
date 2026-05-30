pub fn simple_moving_average(prices: &[f64], window: usize) -> Vec<Option<f64>> {
    let mut sma: Vec<Option<f64>> = Vec::new();

    for i in 0..prices.len() {
        if i + 1 < window {
            sma.push(None);
            continue;
        }

        if let Some(slice) = prices.get(i + 1 - window..i + 1) {
            sma.push(Some(slice.iter().sum::<f64>() / slice.len() as f64));
        }
    }

    sma
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_moving_average_window_3() {
        let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        let result = simple_moving_average(&prices, 3);
        let expected = vec![None, None, Some(11.0), Some(12.0), Some(13.0)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple_moving_average_empty_prices() {
        let prices: Vec<f64> = Vec::new();
        let result = simple_moving_average(&prices, 3);
        assert_eq!(result, Vec::<Option<f64>>::new());
    }

    #[test]
    fn test_simple_moving_average_window_larger_than_prices() {
        let prices = vec![10.0, 11.0];
        let result = simple_moving_average(&prices, 3);
        let expected = vec![None, None];
        assert_eq!(result, expected);
    }
}
