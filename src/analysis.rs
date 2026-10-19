use crate::model::{Day, Summary};

// TODO(part 3): find the real min/max closes and the volatile days (range > 2% of close).
pub fn summarize(days: &[Day]) -> Option<Summary> {
    Some(Summary {
        min: *days.first()?,
        max: *days.last()?,
        volatile: Vec::new(),
    })
}
