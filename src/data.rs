use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PriceBar {
    pub date: String,
    pub symbol: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

pub fn load_price_bars(path: &str) -> Result<Vec<PriceBar>, csv::Error> {
    let mut data = csv::Reader::from_path(path)?;

    let mut bars: Vec<PriceBar> = Vec::new();

    for result in data.deserialize() {
        let bar: PriceBar = result?;
        bars.push(bar);
    }

    Ok(bars)
}

pub fn close_prices(bars: &[PriceBar]) -> Vec<f64> {
    bars.iter().map(|x| x.close).collect()
}
