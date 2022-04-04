pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        0 => 0.0,
        1..=4 => 221.0 * speed as f64,
        5..=8 => 0.9 * 221.0 * speed as f64,
        9..=10 => 0.77 * 221.0 * speed as f64,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
