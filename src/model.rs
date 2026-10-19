use chrono::NaiveDate;

#[derive(Debug, Clone, Copy)]
pub struct Day {
    pub date: NaiveDate,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

#[derive(Debug)]
pub struct Summary {
    pub min: Day,
    pub max: Day,
    pub volatile: Vec<Day>,
}
