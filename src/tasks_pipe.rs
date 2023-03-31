use crate::{
    arithmetic::{Sub, Sum},
    task::Question,
};
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
        let result = question.check(&line)?;
        writeln!(writer, "{}", result);
    }
    Ok(())
}

#[test]
fn sum_and_sub_0() -> Result<()> {
    let mut questions: Vec<Box<dyn Question>> =
        vec![Box::new(Sum { a: 1, b: 1 }), Box::new(Sub { a: 1, b: 1 })];
    let mut input = "2\n0\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run(&questions, &mut input, &mut output);
    assert_eq!(&output, b"1 + 1 = ?\ntrue\n1 - 1 = ?\ntrue\n");
    Ok(())
}

#[test]
fn sum_and_sub_1() -> Result<()> {
    let mut questions: Vec<Box<dyn Question>> =
        vec![Box::new(Sum { a: 1, b: 1 }), Box::new(Sub { a: 1, b: 1 })];
    let mut input = "2\n2\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run(&questions, &mut input, &mut output);
    assert_eq!(&output, b"1 + 1 = ?\ntrue\n1 - 1 = ?\nfalse\n");
    Ok(())
}

#[test]
fn sum_and_sub_fail_on_ono_digit() -> Result<()> {
    let mut questions: Vec<Box<dyn Question>> =
        vec![Box::new(Sum { a: 1, b: 1 }), Box::new(Sub { a: 1, b: 1 })];
    let mut input = "f\n2\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    let res = run(&questions, &mut input, &mut output);
    assert!(res.is_err());
    Ok(())
}
