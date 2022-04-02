pub fn square(s: u32) -> u64 {
    match s {
        x if x > 0 && x < 65 => 2u64.pow(x - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    let mut sum: u64 = 0;
    for i in 1..65 {
        sum += square(i);
    }
    sum
}
