// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub struct Planet {
    yrs_in_earth_yrs: f64,
}

impl Planet {
    fn years_during(&self, d: &Duration) -> f64 {
        d.seconds as f64 / (31557600.0 * self.yrs_in_earth_yrs)
    }
}

pub struct Earth {
    yrs_in_earth_yrs: f64,
}

// pub struct Mercury;
// pub struct Venus;
// pub struct Mars;
// pub struct Jupiter;
// pub struct Saturn;
// pub struct Uranus;
// pub struct Neptune;
//
// impl Planet for Mercury {}
// impl Planet for Venus {}
// impl Planet for Mars {}
// impl Planet for Jupiter {}
// impl Planet for Saturn {}
// impl Planet for Uranus {}
// impl Planet for Neptune {}
