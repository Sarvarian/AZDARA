pub struct Time {
    pub speed: f64,
    counter: f64,
    os: &'static gdnative::api::OS,
    last_tick_time: i64,
}

impl Time {
    pub fn new() -> Self {
        let os = gdnative::api::OS::godot_singleton();
        Time {
            speed: 1f64,
            counter: 1f64,
            os,
            last_tick_time: 0,
        }
    }

    pub fn process(&mut self) -> u8 {
        let mut res = 0u8;
        let this_tick_time = self.os.get_ticks_msec();
        let delta = (this_tick_time - self.last_tick_time) as f64 / 1000f64;
        self.counter -= delta;
        while self.counter < 0f64 {
            self.counter += self.speed;
            res += 1;
        }
        self.last_tick_time = this_tick_time;
        res
    }
}
