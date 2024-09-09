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
        self.minutes == other.minutes && self.hours == self.hours
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
            hours,
            minutes,
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock {
            hours: self.hours + minutes / 60,
            minutes: self.minutes + minutes % 60
        }
    }
}
