use std::path::PathBuf;

use clap::Parser;

/// Charts six months of daily closing prices and highlights volatile days.
#[derive(Parser, Debug)]
#[command(
    name = "stock-monitor",
    version,
    about,
    long_about = "Charts the last six months of daily closing prices for a stock.\n\n\
                  Days where the intraday range (high - low) exceeds 2% of the close are\n\
                  marked as volatile. The minimum and maximum closing prices and their\n\
                  dates are printed, and the chart is saved as a PNG image."
)]
pub struct Cli {
    /// Stock ticker symbol, e.g. AAPL
    #[arg(value_parser = parse_ticker)]
    pub ticker: String,

    /// Where to save the chart [default: <TICKER>_chart.png]
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

impl Cli {
    pub fn output_path(&self) -> PathBuf {
        self.output
            .clone()
            .unwrap_or_else(|| PathBuf::from(format!("{}_chart.png", self.ticker)))
    }
}

fn parse_ticker(s: &str) -> Result<String, String> {
    let valid = !s.is_empty()
        && s.len() <= 10
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || ".-^".contains(c));

    if valid {
        Ok(s.to_ascii_uppercase())
    } else {
        Err("must be 1-10 characters: letters, digits, '.', '-' or '^'".into())
    }
}
