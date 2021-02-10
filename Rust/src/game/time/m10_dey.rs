use super::{m11_bahman::Bahman, MonthTrait, TimeDate};

pub struct Dey {}

impl Dey {
    pub fn new() -> Self {
        Dey {}
    }
}

impl MonthTrait for Dey {
    fn num(&self) -> u8 {
        10
    }

    fn name_fa(&self) -> &'static str {
        todo!()
    }

    fn name_en(&self) -> &'static str {
        todo!()
    }

    fn increment_a_day(&self, timedate: &mut TimeDate) {
        timedate.day += 1;
        if timedate.day > 30 {
            timedate.day -= 30;
            timedate.month = Box::new(Bahman::new());
        }
    }

    fn skip_a_month(&self, timedate: &mut TimeDate) {
        todo!()
    }
}
