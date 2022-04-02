use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: u8,
    minutes: u8,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let new_clock: Clock = Clock {
            hours: 0,
            minutes: 0,
        };

        new_clock.add_minutes((minutes + (hours * 60)) as i64)
    }

    pub fn add_minutes(&self, minutes: i64) -> Self {
        let mut min = minutes + (self.minutes as i64) + ((self.hours as i64) * 60);
        let mut hrs = min / 60;

        // Bare with me here. Since rust is a stupid language for stupid people, it does not have a
        // modulus function. So, now that I have to deal with negatives myself (which is fucking
        // barbaric), I have to use this flag to make sure that the time does not overflow weirdly.
        let mut weird_overflow = false;

        if min < 0 {
            hrs -= 1;
            weird_overflow = true;
        }

        min = ((min % 60) + 60) % 60;

        if min == 0 && weird_overflow {
            hrs += 1;
        }

        hrs = ((hrs % 24) + 24) % 24;

        Clock {
            hours: hrs as u8,
            minutes: min as u8,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
