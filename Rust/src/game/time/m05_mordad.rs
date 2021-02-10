use super::{m06_shahrivar::Shahrivar, MonthTrait, TimeDate};

pub struct Mordad {}

impl Mordad {
    pub fn new() -> Self {
        Mordad {}
    }
}

impl MonthTrait for Mordad {
    fn num(&self) -> u8 {
        5
    }

    fn name_fa(&self) -> &'static str {
        todo!()
    }

    fn name_en(&self) -> &'static str {
        todo!()
    }

    fn increment_a_day(&self, timedate: &mut TimeDate) {
        timedate.day += 1;
        if timedate.day > 31 {
            timedate.day -= 31;
            timedate.month = Box::new(Shahrivar::new());
        }
    }

    fn skip_a_month(&self, timedate: &mut TimeDate) {
        todo!()
    }
}
