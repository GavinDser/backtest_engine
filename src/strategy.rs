use crate::signal::Signal;

pub fn generate_sma_signals(
    short_sma: &[Option<f64>],
    long_sma: &[Option<f64>],
) -> Result<Vec<Signal>, StrategyError> {
    if short_sma.len() != long_sma.len() {
        return Err(StrategyError::LengthMismatch {
            short_len: short_sma.len(),
            long_len: long_sma.len(),
        });
    }
    let mut signal: Vec<Signal> = Vec::new();

    for i in 0..short_sma.len() {
        match (short_sma[i], long_sma[i]) {
            (Some(short), Some(long)) => {
                if short > long {
                    signal.push(Signal::Buy);
                } else if short < long {
                    signal.push(Signal::Sell)
                } else {
                    signal.push(Signal::Hold)
                }
            }
            _ => signal.push(Signal::Hold),
        }
    }

    Ok(signal)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrategyError {
    LengthMismatch { short_len: usize, long_len: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_sma_signals() {
        let short_sma = vec![None, Some(2.0), Some(1.0), Some(3.0), Some(4.0)];

        let long_sma = vec![None, Some(1.0), Some(2.0), Some(3.0), None];

        let expected = vec![
            Signal::Hold, // None
            Signal::Buy,  // 2 > 1
            Signal::Sell, // 1 < 2
            Signal::Hold, // 3 == 3
            Signal::Hold, // long None
        ];
        assert_eq!(
            generate_sma_signals(&short_sma, &long_sma).unwrap(),
            expected
        );
    }

    #[test]
    fn test_generate_sma_signals_length_mismatch() {
        let short_sma = vec![Some(1.0), Some(2.0)];
        let long_sma = vec![Some(1.0)];

        let result = generate_sma_signals(&short_sma, &long_sma);

        assert_eq!(
            result,
            Err(StrategyError::LengthMismatch {
                short_len: 2,
                long_len: 1,
            })
        );
    }
}
