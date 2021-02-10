use gdnative::prelude::*;
use m01_farvardin::Farvardin;
mod m10_dey;
mod m11_bahman;
mod m12_esfand;
mod m12_esfand_leap;
mod m01_farvardin;
mod m02_ordibehesht;
mod m03_khordad;
mod m04_tir;
mod m05_mordad;
mod m06_shahrivar;
mod m07_mehr;
mod m08_aban;
mod m09_azar;

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
        let mut res = [0u8; 6];
        while self.counter < 0f64 {
            self.counter += self.speed;
            res = self.increment_a_second(res);
        }
        owner.emit_signal(
            "time_pass",
            &[Variant::from_byte_array(&TypedArray::from_vec(
                res.to_vec(),
            ))],
        );
        self.last_tick_time = this_tick_time;
    }

    pub fn increment_a_second(&mut self, mut res: [u8; 6]) -> [u8; 6] {
        // Second
        self.second += 1;
        res[0] += 1;
        // Minute
        if !(self.second < 60) {
            self.second -= 60;
            self.minute += 1;
            res[1] += 1;
            // Hour
            if !(self.minute < 60) {
                self.minute -= 60;
                self.hour += 1;
                res[2] += 1;
                // Day
                if !(self.hour < 24) {
                    self.hour -= 24;
                    self.day += 1;
                    res[3] += 1;
                    // Month
                    if self.month < 6 {
                        if self.day == 31 {
                            self.day -= 31;
                            res = self.increment_a_month(res);
                        }
                    } else {
                        if self.month < 11 {
                            if self.day == 30 {
                                self.day -= 30;
                                res = self.increment_a_month(res);
                            }
                        } else {
                            if self.year % 4 == 2 {
                                if self.day == 30 {
                                    self.day -= 30;
                                    res = self.increment_a_month(res);
                                }
                            } else {
                                if self.day == 29 {
                                    self.day -= 29;
                                    res = self.increment_a_month(res);
                                }
                            }
                        }
                    }
                }
            }
        }
        res
    }
    fn increment_a_month(&mut self, mut res: [u8; 6]) -> [u8; 6] {
        // Month
        self.month += 1;
        res[4] += 1;
        // Year
        if !(self.month < 12) {
            self.month -= 12;
            self.year += 1;
            res[5] += 1;
        }
        res
    }
}

pub struct TimeDate {
    second: u8,
    minute: u8,
    hour: u8,
    day: u8,
    month: Box<dyn MonthTrait>,
    year: u8,
}

impl TimeDate {
    fn new() -> Self {
        TimeDate {
            second: 0,
            minute: 0,
            hour: 0,
            day: 1,
            month: Box::new(Farvardin::new()),
            year: 1,
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        vec![
            self.second,
            self.minute,
            self.hour,
            self.day,
            self.month.num(),
            self.year,
        ]
    }

    fn get(&self) -> [u8; 6] {
        [
            self.second,
            self.minute,
            self.hour,
            self.day,
            self.month.num(),
            self.year,
        ]
    }
}

trait MonthTrait {
    fn num(&self) -> u8;
    fn name_fa(&self) -> &'static str;
    fn name_en(&self) -> &'static str;
    fn increment_a_day(&self, timedate: &mut TimeDate);
    fn skip_a_month(&self, timedate: &mut TimeDate);
}
