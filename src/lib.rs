use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hours = hours;
        let mut minutes = minutes;
        while minutes < 0 {
            minutes += 60;
            hours -= 1;
        }
        while minutes > 60 {
            minutes -= 60;
            hours += 1;
        }
        while hours < 0 {
            hours += 24;
        }
        while hours > 24 {
            hours -= 24;
        }
        if minutes == 60 {
            minutes = 0;
            hours += 1;
        }
        if hours == 24 {
            hours = 0;
        }
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {        
        Clock::new(self.hours, self.minutes+minutes)       
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.hours < 10 && self.minutes < 10 {
            write!(f, "0{}:0{}", self.hours, self.minutes)
        } else if self.hours < 10 {
            write!(f, "0{}:{}", self.hours, self.minutes)
        } else if self.minutes < 10 {
            write!(f, "{}:0{}", self.hours, self.minutes)
        } else {
            write!(f, "{}:{}", self.hours, self.minutes)
        }
    }
}
