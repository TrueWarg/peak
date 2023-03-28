use crate::task::Question;
use anyhow::{Ok, Result};
use std::io::{BufRead, Write};

pub fn run<Q: Question + ?Sized>(
    questions: &Vec<Box<Q>>,
    mut reader: impl BufRead,
    mut writer: impl Write,
) -> Result<()> {
    for question in questions {
        let body = question.body();
        writeln!(writer, "{}", body)?;
        let mut line = String::new();
        reader.read_line(&mut line);
        let result = question.check(&line);
        writeln!(writer, "{}", result);
    }
    Ok(())
}
