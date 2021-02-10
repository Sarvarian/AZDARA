use super::{m10_dey::Dey, MonthTrait, TimeDate};

pub struct Azar {}

impl Azar {
    pub fn new() -> Self {
        Azar {}
    }
}

impl MonthTrait for Azar {
    fn num(&self) -> u8 {
        9
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
            timedate.month = Box::new(Dey::new());
        }
    }

    fn skip_a_month(&self, timedate: &mut TimeDate) {
        todo!()
    }
}
