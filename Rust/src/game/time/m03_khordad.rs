use super::{m04_tir::Tir, MonthTrait, TimeDate};

pub struct Khordad {}

impl Khordad {
    pub fn new() -> Self {
        Khordad {}
    }
}

impl MonthTrait for Khordad {
    fn num(&self) -> u8 {
        3
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
            timedate.month = Box::new(Tir::new());
        }
    }

    fn skip_a_month(&self, timedate: &mut TimeDate) {
        todo!()
    }
}
