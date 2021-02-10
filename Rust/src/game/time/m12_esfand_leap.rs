use super::{m01_farvardin::Farvardin, MonthTrait, TimeDate};

pub struct EsfandLeap {}

impl EsfandLeap {
    pub fn new() -> Self {
        EsfandLeap {}
    }
}

impl MonthTrait for EsfandLeap {
    fn num(&self) -> u8 {
        12
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
            timedate.month = Box::new(Farvardin::new());
        }
    }

    fn skip_a_month(&self, timedate: &mut TimeDate) {
        todo!()
    }
}
