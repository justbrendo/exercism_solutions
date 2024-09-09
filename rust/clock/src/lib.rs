use std::fmt::{Debug, Display};

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes && self.hours == other.hours
    }
}

impl Debug for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}",self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: hours.rem_euclid(24),
            minutes: 0,
        }.add_minutes(minutes) // We call add minutes here to update the clock and treat negative minutes
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        let n_minutes = (self.hours * 60 + self.minutes + minutes).rem_euclid(1440);
        
        Clock {
            hours: n_minutes / 60,
            minutes: n_minutes.rem_euclid(60),
        }
    }
}
