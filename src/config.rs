#[derive(Debug, Clone, Copy)]
pub struct BacktestConfig {
    pub initial_cash: f64,
    pub commission_rate: f64,
}

impl BacktestConfig {
    pub fn new(initial_cash: f64, commission_rate: f64) -> Result<Self, ConfigError> {
        if initial_cash < 0.0 || commission_rate < 0.0 {
            return Err(ConfigError::InvalidValueError);
        }

        Ok(BacktestConfig {
            initial_cash,
            commission_rate,
        })
    }
}

pub enum ConfigError {
    InvalidValueError,
}
