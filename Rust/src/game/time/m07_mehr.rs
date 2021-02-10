use super::{m08_aban::Aban, MonthTrait, TimeDate};

pub struct Mehr {}

impl Mehr {
    pub fn new() -> Self {
        Mehr {}
    }
}

impl MonthTrait for Mehr {
    fn num(&self) -> u8 {
        7
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
            timedate.month = Box::new(Aban::new());
        }
    }

    fn skip_a_month(&self, timedate: &mut TimeDate) {
        todo!()
    }
}
