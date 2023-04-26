use crate::task::Question;
use anyhow::{anyhow, Context, Ok, Result};

// first draft implementation

#[derive(Debug, PartialEq, Eq)]
pub struct Missing {
    pub items: Vec<SeqItem>,
    pub options: Vec<SeqItem>,
    pub solution: SeqItem,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SeqItem {
    pub form: Form,
    pub size: Size,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Form {
    Star,
    Ampersand,
    Cover,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Size {
    Small,
    Medium,
    Big,
}

pub trait OrderedEnum {
    fn by_index(index: u8) -> Self;
}

impl OrderedEnum for Form {
    fn by_index(index: u8) -> Form {
        match index {
            0 => Form::Star,
            1 => Form::Ampersand,
            2 => Form::Cover,
            _ => panic!("Index out of bound {}", index),
        }
    }
}

impl OrderedEnum for Size {
    fn by_index(index: u8) -> Size {
        match index {
            0 => Size::Small,
            1 => Size::Medium,
            2 => Size::Big,
            _ => panic!("Index out of bound {}", index),
        }
    }
}

pub fn all_combinations() -> Vec<SeqItem> {
    let mut result: Vec<SeqItem> = vec![];
    for i in 0..3 {
        for j in 0..3 {
            result.push(SeqItem {
                form: Form::by_index(i),
                size: Size::by_index(j),
            })
        }
    }
    return result;
}

impl Question for Missing {
    fn body(&self) -> String {
        let mut sequence = String::from("");
        let size = self.items.len();
        for (pos, item) in self.items.iter().enumerate() {
            if item == &self.solution {
                sequence += "?"
            } else {
                sequence += &figure(item);
            }
            if pos < size - 1 {
                sequence += " "
            }
        }
        let mut options = String::from("");
        let size = self.options.len();
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
            .with_context(|| format!("Input is not an integer `{}`", answer.trim()))?;
        let answer = self
            .options
            .get(answer as usize - 1)
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
