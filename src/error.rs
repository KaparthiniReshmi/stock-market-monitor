use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Fetch(String),
    NoData(String),
    Plot(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fetch(reason) => write!(f, "failed to fetch quotes: {reason}"),
            Self::NoData(ticker) => write!(f, "no quote data available for {ticker}"),
            Self::Plot(reason) => write!(f, "failed to draw chart: {reason}"),
        }
    }
}

impl std::error::Error for AppError {}
