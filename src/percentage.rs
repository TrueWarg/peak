use crate::task::Question;
use anyhow::{Context, Ok, Result};

#[derive(Debug, PartialEq)]
pub struct Percent {
    pub full: f64,
    pub percent: f64,
    pub precision: u8,
}

impl Question for Percent {
    fn body(&self) -> String {
        let full = format!("{} = 100 %", self.full);
        let percent = format!("? ~= {} %", self.percent);
        return format!("{}\n{}", full, percent);
    }

    fn check(&self, answer: &String) -> Result<bool> {
        let answer: f64 = answer
            .trim()
            .parse()
            .with_context(|| format!("Input is not an real `{}`", answer.trim()))?;
        let solution = (self.full * self.percent) / 100.0;
        Ok(approx_equal(answer, solution, self.precision))
    }
}

fn approx_equal(a: f64, b: f64, decimal_places: u8) -> bool {
    let factor = 10.0f64.powi(decimal_places as i32);
    let a = (a * factor).trunc();
    let b = (b * factor).trunc();
    a == b
}
