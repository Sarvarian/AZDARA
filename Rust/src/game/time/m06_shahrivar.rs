use super::{m07_mehr::Mehr, MonthTrait, TimeDate};

pub struct Shahrivar {}

impl Shahrivar {
    pub fn new() -> Self {
        Shahrivar {}
    }
}

impl MonthTrait for Shahrivar {
    fn num(&self) -> u8 {
        6
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
            timedate.month = Box::new(Mehr::new());
        }
    }

    fn skip_a_month(&self, timedate: &mut TimeDate) {
        todo!()
    }
}
