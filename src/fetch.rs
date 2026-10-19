use chrono::NaiveDate;

use crate::error::AppError;
use crate::model::Day;

// TODO(part 2): fetch 6 months of daily quotes from Yahoo Finance.
pub fn fetch_quotes(_ticker: &str) -> Result<Vec<Day>, AppError> {
    let days = (1..=5)
        .filter_map(|d| NaiveDate::from_ymd_opt(2026, 4, d).map(|date| (date, f64::from(d))))
        .map(|(date, x)| Day {
            date,
            open: 100.0 + x,
            high: 102.0 + x,
            low: 99.0 + x,
            close: 101.0 + x,
        })
        .collect();
    Ok(days)
}
