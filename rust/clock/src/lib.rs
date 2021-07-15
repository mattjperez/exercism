use std::fmt;

const MIN_PER_DAY: i32 = 1440;
const MIN_PER_HOUR: i32 = 60;
const HOUR_PER_DAY: i32 = 24;

#[derive(Debug, Default, PartialEq)]
pub struct Clock {
    min: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hour = self.min / MIN_PER_HOUR;
        let min = self.min % MIN_PER_HOUR;
        write!(f, "{:02}:{:02}", hour % HOUR_PER_DAY, min)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hour = hours + (minutes / MIN_PER_HOUR);

        let mut min = if minutes % MIN_PER_HOUR < 0 {
            hour -= 1;
            MIN_PER_HOUR + (minutes % MIN_PER_HOUR)
        } else {
            minutes % MIN_PER_HOUR
        };

        hour = if hour % HOUR_PER_DAY < 0 {
            (hour % HOUR_PER_DAY) + HOUR_PER_DAY
        } else {
            hour % HOUR_PER_DAY
        };

        min += hour * MIN_PER_HOUR;
        min %= MIN_PER_DAY;

        Clock { min }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let sum = self.min + minutes;
        let min = if sum < 0 {
            (sum % MIN_PER_DAY) + MIN_PER_DAY
        } else {
            sum
        };

        Clock { min }
    }
}
