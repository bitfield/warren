use std::{cmp::Ordering, fmt::Display};

use anyhow::{Context, Result};
use tokio::runtime::Runtime;
use yahoo_finance_api as yahoo;

pub struct Report {
    symbol: String,
    price: f64,
    low: f64,
    high: f64,
    position: f64,
    recommendation: String,
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: Price {:.2} Low {:.2} High {:.2} Position {:.2}% - {}",
            self.symbol, self.price, self.low, self.high, self.position, self.recommendation
        )
    }
}

/// Returns a quote history analysis on `stock`.
///
/// # Errors
///
/// Returns any errors creating the Yahoo Finance connector, fetching the quote
/// history, extracting the last quote, or finding the minimum and maximum for
/// the period.
///
/// # Panics
///
/// If the current price is outside the range (low, high).
pub fn get_report(stock: &str) -> Result<Report> {
    let provider = yahoo::YahooConnector::new()?;
    let rt = Runtime::new().unwrap();
    let resp = rt.block_on(provider.get_quote_range(stock, "1d", "3mo"))?;
    let price = resp.last_quote()?.close;
    let high = resp
        .quotes()?
        .iter()
        .map(|q| q.high)
        .max_by(|x, y| x.partial_cmp(y).unwrap_or(Ordering::Less))
        .context("bad data")?;
    let low = resp
        .quotes()?
        .iter()
        .map(|q| q.low)
        .min_by(|x, y| x.partial_cmp(y).unwrap_or(Ordering::Greater))
        .context("bad data")?;
    let range = high - low;
    let position = (price - low) / range * 100.0;
    let recommendation = recommend(position);
    Ok(Report {
        symbol: stock.to_owned(),
        price,
        low,
        high,
        position,
        recommendation,
    })
}

fn recommend(position: f64) -> String {
    match position {
        0.0..50.0 => "Buy".to_string(),
        50.0..100.0 => "Don't buy".to_string(),
        p => unreachable!("bad percentage {p}"),
    }
}

#[cfg(test)]
mod tests {   
   use super::*;
   
    #[test]
    fn recommend_returns_correct_recommendation_for_given_position() {
        assert_eq!(recommend(45.0), "Buy");
    }
}