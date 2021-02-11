use gdnative::prelude::*;

pub struct Time {
    counter: f64,
    pub speed: f64,
    os: &'static gdnative::api::OS,
    last_tick_time: i64,
    timedate: TimeDate,
    date_state: Box<dyn DateState>,
}

impl Time {
    pub fn new() -> Self {
        let os = gdnative::api::OS::godot_singleton();
        Time {
            counter: 1f64,
            speed: 1f64,
            os,
            last_tick_time: 0,
            timedate: TimeDate::new(),
            date_state: Box::new(FirstHalf),
        }
    }

    pub fn timedate(&self) -> &TimeDate {
        &self.timedate
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
        self.timedate.second += 1;
        res[0] += 1;
        // Minute
        if !(self.timedate.second < 60) {
            self.timedate.second -= 60;
            self.timedate.minute += 1;
            res[1] += 1;
            // Hour
            if !(self.timedate.minute < 60) {
                self.timedate.minute -= 60;
                self.timedate.hour += 1;
                res[2] += 1;
                // Day
                self.date_state.increment_a_day();
            }
        }
        res
    }
}

trait DateState {
    fn increment_a_day(&self, time: &mut Time);
    fn skip_a_month(&self, time: &mut Time);
}

struct FirstHalf;

impl DateState for FirstHalf {
    fn increment_a_day(&self, time: &mut Time) {
        time.timedate.day += 1;
        if time.timedate.day > 31 {
            time.timedate.day -= 31;
            time.timedate.month += 1;
            if time.timedate.month > 6 {
                time.date_state = Box::new(SecondHalf);
            }
        }
    }

    fn skip_a_month(&self, timedate: &mut Time) {
        todo!()
    }
}

struct SecondHalf;

impl DateState for SecondHalf {
    fn increment_a_day(&self, time: &mut Time) {
        time.timedate.day += 1;
        if time.timedate.day > 30 {
            time.timedate.day -= 30;
            time.timedate.month += 1;
            if time.timedate.month > 11 {
                if time.timedate.year % 4 == 3 {
                    time.date_state = Box::new(EsfandLeap);
                }
                time.date_state = Box::new(Esfand);
            }
        }
    }

    fn skip_a_month(&self, time: &mut Time) {
        todo!()
    }
}

struct Esfand;

impl DateState for Esfand {
    fn increment_a_day(&self, time: &mut Time) {
        time.timedate.day += 1;
        if time.timedate.day > 29 {
            time.timedate.day -= 29;
            time.timedate.month += 1;
            if time.timedate.month > 6 {
                time.date_state = Box::new(FirstHalf);
            }
        }
    }

    fn skip_a_month(&self, time: &mut Time) {
        todo!()
    }
}

struct EsfandLeap;

impl DateState for EsfandLeap {
    fn increment_a_day(&self, time: &mut Time) {
        time.timedate.day += 1;
        if time.timedate.day > 30 {
            time.timedate.day -= 30;
            time.timedate.month += 1;
            if time.timedate.month > 6 {
                time.date_state = Box::new(FirstHalf);
            }
        }
    }

    fn skip_a_month(&self, time: &mut Time) {
        todo!()
    }
}

struct TimeDate {
    second: u8,
    minute: u8,
    hour: u8,
    day: u8,
    month: u8,
    year: u8,
}

impl TimeDate {
    fn new() -> Self {
        TimeDate {
            second: 0,
            minute: 0,
            hour: 0,
            day: 1,
            month: 1,
            year: 1,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        vec![
            self.second,
            self.minute,
            self.hour,
            self.day,
            self.month,
            self.year,
        ]
    }

    pub fn arr(&self) -> [u8; 6] {
        [
            self.second,
            self.minute,
            self.hour,
            self.day,
            self.month,
            self.year,
        ]
    }
}
