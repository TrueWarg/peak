use crate::task::Question;
use anyhow::{Context, Ok, Result};

// first draft implementation

#[derive(Debug, PartialEq, Eq)]
pub struct Missing {
    items: Vec<SeqItem>,
    options: Vec<SeqItem>,
    answer: SeqItem,
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
