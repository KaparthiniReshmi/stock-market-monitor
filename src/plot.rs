use std::path::Path;

use crate::error::AppError;
use crate::model::{Day, Summary};

// TODO(part 4): draw the chart with plotters and save it to `out`.
pub fn draw_chart(
    _ticker: &str,
    _days: &[Day],
    _summary: &Summary,
    out: &Path,
) -> Result<(), AppError> {
    std::fs::copy("assets/placeholder_chart.png", out)
        .map_err(|e| AppError::Plot(e.to_string()))?;
    Ok(())
}
