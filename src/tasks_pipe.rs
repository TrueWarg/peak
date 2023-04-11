use crate::{
    arithmetic::{Div, Mod, Mul, Sub, Sum},
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
fn sum_and_sub_fail_on_non_digit() -> Result<()> {
    let mut questions: Vec<Box<dyn Question>> =
        vec![Box::new(Sum { a: 1, b: 1 }), Box::new(Sub { a: 1, b: 1 })];
    let mut input = "f\n2\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    let res = run(&questions, &mut input, &mut output);
    assert!(res.is_err());
    Ok(())
}

#[test]
fn mul_and_div_0() -> Result<()> {
    let mut questions: Vec<Box<dyn Question>> =
        vec![Box::new(Mul { a: 4, b: 5 }), Box::new(Div { a: 5, b: 2 })];
    let mut input = "20\n3\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run(&questions, &mut input, &mut output);
    assert_eq!(&output, b"4 * 5 = ?\ntrue\n5 div 2 = ?\nfalse\n");
    Ok(())
}

#[test]
fn div_and_mod_0() -> Result<()> {
    let mut questions: Vec<Box<dyn Question>> =
        vec![Box::new(Div { a: 8, b: 4 }), Box::new(Mod { a: 5, b: 2 })];
    let mut input = "2\n1\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run(&questions, &mut input, &mut output);
    assert_eq!(&output, b"8 div 4 = ?\ntrue\n5 mod 2 = ?\ntrue\n");
    Ok(())
}
