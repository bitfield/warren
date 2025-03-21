use std::fmt::Display;

use anyhow::Result;
use tokio::runtime::Runtime;
use yahoo_finance_api::{Quote, YahooConnector};

pub struct Report {
    pub symbol: String,
    pub history: QuoteHistory,
    pub position: f64,
    pub recommendation: String,
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: Current {:.2} Low {:.2} High {:.2} Position {:.2}% - {}",
            self.symbol,
            self.history.current,
            self.history.low,
            self.history.high,
            self.position,
            self.recommendation
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
    let h = quote_history(stock)?;
    let range = h.high - h.low;
    let position = (h.current - h.low) / range * 100.0;
    let recommendation = recommend(position);
    Ok(Report {
        symbol: stock.to_owned(),
        history: h,
        position,
        recommendation,
    })
}

#[derive(Debug)]
pub struct QuoteHistory {
    pub high: f64,
    pub low: f64,
    pub current: f64,
}

impl Default for QuoteHistory {
    fn default() -> Self {
        Self {
            high: 0.0,
            low: f64::MAX,
            current: 0.0,
        }
    }
}

/// Returns price history statistics for `stock` over the last 90 days.
///
/// # Errors
///
/// Any errors returned from contacting the Yahoo Finance API.
pub fn quote_history(stock: &str) -> Result<QuoteHistory> {
    let provider = YahooConnector::new()?;
    let rt = Runtime::new()?;
    let resp = rt.block_on(provider.get_quote_range(stock, "1d", "3mo"))?;
    Ok(stats(&resp.quotes()?))
}

fn recommend(position: f64) -> String {
    match position {
        0.0..50.0 => "Buy".to_string(),
        50.0..100.0 => "Don't buy".to_string(),
        p => unreachable!("bad percentage {p}"),
    }
}

fn stats(quotes: &[Quote]) -> QuoteHistory {
    let mut h = QuoteHistory::default();
    for q in quotes {
        h.current = q.close;
        if q.high > h.high {
            h.high = q.high;
        }
        if q.low < h.low {
            h.low = q.low;
        }
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recommend_returns_correct_recommendation_for_given_position() {
        assert_eq!(recommend(45.0), "Buy");
        assert_eq!(recommend(55.0), "Don't buy");
    }

    #[test]
    #[allow(
        clippy::float_cmp,
        reason = "answers are chosen from input, not computed"
    )]
    fn stats_returns_correct_stats_for_quote_series() {
        let input = vec![
            Quote {
                timestamp: 0,
                open: 0.0,
                high: 10.0,
                low: 2.0,
                volume: 0,
                close: 0.0,
                adjclose: 0.0,
            },
            Quote {
                timestamp: 0,
                open: 0.0,
                high: 5.0,
                low: 1.0,
                volume: 0,
                close: 4.0,
                adjclose: 0.0,
            },
        ];
        let s = stats(&input);
        assert_eq!(s.current, 4.0, "wrong current");
        assert_eq!(s.high, 10.0, "wrong high");
        assert_eq!(s.low, 1.0, "wrong low");
    }

    #[test]
    fn report_impls_display_correctly() {
        let report = Report {
            symbol: "AAPL".into(),
            history: QuoteHistory {
                high: 4.0,
                low: 1.0,
                current: 2.0,
            },
            position: 25.0,
            recommendation: "Buy".into(),
        };
        assert_eq!(
            format!("{report}"),
            "AAPL: Current 2.00 Low 1.00 High 4.00 Position 25.00% - Buy"
        );
    }
}
