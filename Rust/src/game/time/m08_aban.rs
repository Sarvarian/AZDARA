use super::{m09_azar::Azar, MonthTrait, TimeDate};

pub struct Aban {}

impl Aban {
    pub fn new() -> Self {
        Aban {}
    }
}

impl MonthTrait for Aban {
    fn num(&self) -> u8 {
        8
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
            timedate.month = Box::new(Azar::new());
        }
    }

    fn skip_a_month(&self, timedate: &mut TimeDate) {
        todo!()
    }
}
