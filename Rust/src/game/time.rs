use gdnative::prelude::*;

pub struct Time {
    counter: f64,
    pub speed: f64,
    os: &'static gdnative::api::OS,
    last_tick_time: i64,
    // pub timedate: Vec<u8>,
    second: u8,
    minute: u8,
    hour: u8,
    day: u8,
    month: u8,
    year: u8,
}

impl Time {
    pub fn new() -> Self {
        let os = gdnative::api::OS::godot_singleton();
        Time {
            counter: 1f64,
            speed: 1f64,
            os,
            last_tick_time: 0,
            // timedate: vec![0,0,0,0,0,0]
            second: 0,
            minute: 0,
            hour: 0,
            day: 0,
            month: 0,
            year: 0,
        }
    }

    pub fn get(&self) -> [u8; 6] {
        [
            self.second,
            self.minute,
            self.hour,
            self.day,
            self.month,
            self.year,
        ]
    }

    pub fn process(&mut self, owner: &Node) {
        let this_tick_time = self.os.get_ticks_msec();
        let delta = (this_tick_time - self.last_tick_time) as f64 / 1000f64;
        self.counter -= delta;
        while self.counter < 0f64 {
            self.counter += self.speed;
            self.add_second(owner);
        }
        self.last_tick_time = this_tick_time;
    }

    fn add_second(&mut self, owner: &Node) {
        // Second
        self.second += 1;
        owner.emit_signal("second_pass", &[]);
        // Minute
        if !(self.second < 60) {
            self.second -= 60;
            self.minute += 1;
            owner.emit_signal("minute_pass", &[]);
            // Hour
            if !(self.minute < 60) {
                self.minute -= 60;
                self.hour += 1;
                owner.emit_signal("hour_pass", &[]);
                // Day
                if !(self.hour < 24) {
                    self.hour -= 24;
                    self.day += 1;
                    owner.emit_signal("day_pass", &[]);
                    // Month
                    if !(self.day < 30) {
                        self.day -= 30;
                        self.month += 1;
                        owner.emit_signal("emit_pass", &[]);
                        // Year
                        if !(self.month < 12) {
                            self.month -= 12;
                            self.year += 1;
                            owner.emit_signal("emit_pass", &[]);
                        }
                    }
                }
            }
        }
    }

    pub fn skip_a_second(&mut self) {
        // Second
        self.second += 1;
        // Minute
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
                        if !(self.month < 12) {
                            self.month -= 12;
                            self.year += 1;
                        }
                    }
                }
            }
        }
    }
}
