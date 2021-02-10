use super::{m01_farvardin::Farvardin, MonthTrait, TimeDate};

pub struct Esfand {}

impl Esfand {
    pub fn new() -> Self {
        Esfand {}
    }
}

impl MonthTrait for Esfand {
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
        if timedate.day > 29 {
            timedate.day -= 29;
            timedate.month = Box::new(Farvardin::new());
            timedate.year += 1;
        }
    }

    fn skip_a_month(&self, timedate: &mut TimeDate) {
        todo!()
    }
}
