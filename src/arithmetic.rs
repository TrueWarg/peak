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

pub struct Mul {
    pub a: i32,
    pub b: i32,
}

pub struct Div {
    pub a: i32,
    pub b: i32,
}

pub struct Mod {
    pub a: i32,
    pub b: i32,
}

impl Question for Sum {
    fn body(&self) -> String {
        return format!("{} + {} = ?", self.a, self.b);
    }

    fn check(&self, answer: &String) -> Result<bool> {
        let answer = convert_to_i32(answer)?;
        let solution = self.a + self.b;
        Ok(answer == solution)
    }
}

impl Question for Sub {
    fn body(&self) -> String {
        return format!("{} - {} = ?", self.a, self.b);
    }

    fn check(&self, answer: &String) -> Result<bool> {
        let answer = convert_to_i32(answer)?;
        let solution = self.a - self.b;
        Ok(answer == solution)
    }
}

impl Question for Mul {
    fn body(&self) -> String {
        return format!("{} * {} = ?", self.a, self.b);
    }

    fn check(&self, answer: &String) -> Result<bool> {
        let answer = convert_to_i32(answer)?;
        let solution = self.a * self.b;
        Ok(answer == solution)
    }
}

impl Question for Div {
    fn body(&self) -> String {
        return format!("{} div {} = ?", self.a, self.b);
    }

    fn check(&self, answer: &String) -> Result<bool> {
        let answer = convert_to_i32(answer)?;
        let solution = self.a / self.b;
        Ok(answer == solution)
    }
}

impl Question for Mod {
    fn body(&self) -> String {
        return format!("{} mod {} = ?", self.a, self.b);
    }

    fn check(&self, answer: &String) -> Result<bool> {
        let answer = convert_to_i32(answer)?;
        let solution = self.a % self.b;
        Ok(answer == solution)
    }
}

fn convert_to_i32(value: &String) -> Result<i32> {
    let value: i32 = value
        .trim()
        .parse()
        .with_context(|| format!("Input is not an integer `{}`", value.trim()))?;
    return Ok(value);
}
