use super::{m05_mordad::Mordad, MonthTrait, TimeDate};

pub struct Tir {}

impl Tir {
    pub fn new() -> Self {
        Tir {}
    }
}

impl MonthTrait for Tir {
    fn num(&self) -> u8 {
        4
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
            timedate.month = Box::new(Mordad::new());
        }
    }

    fn skip_a_month(&self, timedate: &mut TimeDate) {
        todo!()
    }
}
