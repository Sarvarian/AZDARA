use super::{m12_esfand::Esfand, m12_esfand_leap::EsfandLeap, MonthTrait, TimeDate};

pub struct Bahman {}

impl Bahman {
    pub fn new() -> Self {
        Bahman {}
    }
}

impl MonthTrait for Bahman {
    fn num(&self) -> u8 {
        11
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
            if timedate.year % 4 == 3 {
                timedate.month = Box::new(EsfandLeap::new());
            } else {
                timedate.month = Box::new(Esfand::new());
            }
        }
    }

    fn skip_a_month(&self, timedate: &mut TimeDate) {
        todo!()
    }
}
