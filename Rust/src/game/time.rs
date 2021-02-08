pub struct Time {
    counter: f64,
    speed: f64,
    pub timedate: TimaDate,
}

impl Time {
    pub fn new() -> Self {
        Time {
            counter: 1f64,
            speed: 1f64,
            timedate: TimaDate::new(),
        }
    }

    pub fn process(&mut self, delta: f64) {
        self.counter -= delta;
        while !(self.counter > 0f64) {
            self.counter = self.speed - self.counter;
            self.timedate.add_second()
        }
    }
}

pub struct TimaDate {
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: u8,
    pub year: u8,
}

impl TimaDate {
    fn new() -> Self {
        TimaDate {
            second: 0,
            minute: 0,
            hour: 0,
            day: 0,
            month: 0,
            year: 0,
        }
    }

    fn add_second(&mut self) {
        self.second += 1;
        if !(self.second < 60) {
            self.second -= 60;
            self.minute += 1;
            // Hour
            if !(self.minute < 60) {
                self.minute -= 60;
                self.hour += 1;
                // Day
                if !(self.hour < 24) {
                    self.hour -= 24;
                    self.day += 1;
                    // Month
                    if !(self.day < 30) {
                        self.day -= 30;
                        self.month += 1;
                        // Year
                        if !(self.month < 30) {
                            self.month -= 30;
                            self.year += 1;
                        }
                    }
                }
            }
        }
    }
}
