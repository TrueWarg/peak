use crate::task::Question;
use anyhow::{anyhow, Context, Ok, Result};

// first draft implementation

#[derive(Debug, PartialEq, Eq)]
pub struct Missing {
    items: Vec<SeqItem>,
    options: Vec<SeqItem>,
    solution: SeqItem,
}

#[derive(Debug, PartialEq, Eq)]
struct SeqItem {
    form: Form,
    size: Size,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Form {
    Star,
    Ampersand,
    Cover,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Size {
    Small,
    Medium,
    Big,
}

impl Question for Missing {
    fn body(&self) -> String {
        let mut sequence = String::from("");
        let size = self.items.len();
        for (pos, item) in self.items.iter().enumerate() {
            sequence += &figure(item);
            if pos < size - 1 {
                sequence += " "
            }
        }
        let mut options = String::from("");
        for (pos, item) in self.options.iter().enumerate() {
            options += &format!("{}. {}", pos + 1, figure(item));
            if pos < size - 1 {
                options += " "
            }
        }
        return sequence + "\n" + &options;
    }

    fn check(&self, answer: &String) -> Result<bool> {
        let answer: u8 = answer
            .trim()
            .parse()
            .with_context(|| format!("Input is not an integer `{}`", answer))?;
        let answer = self
            .options
            .get(answer as usize)
            .ok_or(anyhow!(format!("Input is not in ranage {}", answer)))?;
        return Ok(answer == &self.solution);
    }
}

fn figure(item: &SeqItem) -> String {
    let symbol = match item.form {
        Form::Star => "*",
        Form::Ampersand => "&",
        Form::Cover => "^",
    };
    let number = match item.size {
        Size::Small => 1,
        Size::Medium => 2,
        Size::Big => 3,
    };
    return symbol.repeat(number);
}
