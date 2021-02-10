use super::{m03_khordad::Khordad, MonthTrait, TimeDate};

pub struct Ordibehesht {}

impl Ordibehesht {
    pub fn new() -> Self {
        Ordibehesht {}
    }
}

impl MonthTrait for Ordibehesht {
    fn num(&self) -> u8 {
        2
    }

    fn name_fa(&self) -> &'static str {
        "اردیبهشت"
    }

    fn name_en(&self) -> &'static str {
        "Ordibehesht"
    }

    fn increment_a_day(&self, timedate: &mut TimeDate) {
        timedate.day += 1;
        if timedate.day > 31 {
            timedate.day -= 31;
            timedate.month = Box::new(Khordad::new());
        }
    }

    fn skip_a_month(&self, timedate: &mut TimeDate) {
        todo!()
    }
}
