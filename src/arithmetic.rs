use crate::task::Question;
use anyhow::{Context, Ok, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct Sum {
    pub a: i32,
    pub b: i32,
}

pub struct Sub {
    pub a: i32,
    pub b: i32,
}

impl Question for Sum {
    fn body(&self) -> String {
        return format!("{} + {} = ?", self.a, self.b);
    }

    fn check(&self, answer: &String) -> Result<bool> {
        let answer: i32 = answer
            .trim()
            .parse()
            .with_context(|| format!("Input is not an integer `{}`", answer))?;
        let solution = self.a + self.b;
        Ok(answer == solution)
    }
}

impl Question for Sub {
    fn body(&self) -> String {
        return format!("{} - {} = ?", self.a, self.b);
    }

    fn check(&self, answer: &String) -> Result<bool> {
        let answer: i32 = answer
            .trim()
            .parse()
            .with_context(|| format!("Input is not an integer `{}`", answer))?;
        let solution = self.a - self.b;
        Ok(answer == solution)
    }
}
