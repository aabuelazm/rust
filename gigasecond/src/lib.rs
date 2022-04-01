use time::{ext::NumericalDuration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.saturating_add(1000000000i64.seconds())
}
