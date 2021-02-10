use super::{m02_ordibehesht::Ordibehesht, MonthTrait, TimeDate};

pub struct Farvardin {}

impl Farvardin {
    pub fn new() -> Self {
        Farvardin {}
    }
}

impl MonthTrait for Farvardin {
    fn num(&self) -> u8 {
        1
    }

    fn name_fa(&self) -> &'static str {
        "فروردین"
    }

    fn name_en(&self) -> &'static str {
        "Farvardin"
    }

    fn increment_a_day(&self, timedate: &mut TimeDate) {
        timedate.day += 1;
        if timedate.day > 31 {
            timedate.day -= 31;
            timedate.month = Box::new(Ordibehesht::new());
        }
    }

    fn skip_a_month(&self, timedate: &mut TimeDate) {
        todo!()
    }
}
