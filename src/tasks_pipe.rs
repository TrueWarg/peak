use crate::{
    abstract_sequence::{all_combinations, Form, Missing, SeqItem, Size},
    arithmetic::{Div, Mod, Mul, Sub, Sum},
    percentage::Percent,
    task::Question,
};
use anyhow::{Ok, Result};
use std::{
    io::{BufRead, Write},
    iter::Skip,
};

#[derive(Debug, PartialEq, Eq)]
pub enum PipeMod {
    UntilRight,
    Skip,
}

pub fn run<Q: Question + ?Sized>(
    questions: &Vec<Box<Q>>,
    pipe_mod: &PipeMod,
    mut reader: impl BufRead,
    mut writer: impl Write,
) -> Result<()> {
    if questions.is_empty() {
        return Ok(());
    }
    let mut index = 0;
    let length = questions.len();
    loop {
        let question = &questions[index];
        let body = question.body();
        writeln!(writer, "{}", body)?;
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let result = question.check(&line);
        match result {
            Result::Ok(correct) => {
                writeln!(writer, "{}", correct)?;
                index = next_index(index, correct, pipe_mod);
            }
            Err(error) => {
                writeln!(writer, "{}", error)?;
            }
        }
        if index == length {
            break;
        }
    }
    Ok(())
}

fn next_index(index: usize, correct: bool, pipe_mod: &PipeMod) -> usize {
    return match pipe_mod {
        PipeMod::UntilRight => {
            if correct {
                index + 1
            } else {
                index
            }
        }
        PipeMod::Skip => index + 1,
    };
}

#[test]
fn sum_and_sub_0() -> Result<()> {
    let questions: Vec<Box<dyn Question>> =
        vec![Box::new(Sum { a: 1, b: 1 }), Box::new(Sub { a: 1, b: 1 })];
    let mut input = "2\n0\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run(&questions, &PipeMod::Skip, &mut input, &mut output)?;
    assert_eq!(&output, b"1 + 1 = ?\ntrue\n1 - 1 = ?\ntrue\n");
    Ok(())
}

#[test]
fn sum_and_sub_1() -> Result<()> {
    let questions: Vec<Box<dyn Question>> =
        vec![Box::new(Sum { a: 1, b: 1 }), Box::new(Sub { a: 1, b: 1 })];
    let mut input = "2\n2\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run(&questions, &PipeMod::Skip, &mut input, &mut output)?;
    assert_eq!(&output, b"1 + 1 = ?\ntrue\n1 - 1 = ?\nfalse\n");
    Ok(())
}

#[test]
fn sum_fail_on_non_digit() -> Result<()> {
    let questions: Vec<Box<dyn Question>> = vec![Box::new(Sum { a: 1, b: 1 })];
    let mut input = "kek\n2\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run(&questions, &PipeMod::Skip, &mut input, &mut output)?;
    assert_eq!(
        &output,
        b"1 + 1 = ?\nInput is not an integer `kek`\n1 + 1 = ?\ntrue\n"
    );
    Ok(())
}

#[test]
fn mul_and_div_0() -> Result<()> {
    let questions: Vec<Box<dyn Question>> =
        vec![Box::new(Mul { a: 4, b: 5 }), Box::new(Div { a: 5, b: 2 })];
    let mut input = "20\n3\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run(&questions, &PipeMod::Skip, &mut input, &mut output)?;
    assert_eq!(&output, b"4 * 5 = ?\ntrue\n5 div 2 = ?\nfalse\n");
    Ok(())
}

#[test]
fn div_and_mod_0() -> Result<()> {
    let questions: Vec<Box<dyn Question>> =
        vec![Box::new(Div { a: 8, b: 4 }), Box::new(Mod { a: 5, b: 2 })];
    let mut input = "2\n1\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run(&questions, &PipeMod::Skip, &mut input, &mut output)?;
    assert_eq!(&output, b"8 div 4 = ?\ntrue\n5 mod 2 = ?\ntrue\n");
    Ok(())
}

#[test]
fn percents_0() -> Result<()> {
    let questions: Vec<Box<dyn Question>> = vec![
        Box::new(Percent {
            full: 100.0,
            percent: 12.0,
            precision: 1,
        }),
        Box::new(Percent {
            full: 200.0,
            percent: 12.0,
            precision: 1,
        }),
    ];
    let mut input = "12\n12\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run(&questions, &PipeMod::Skip, &mut input, &mut output)?;
    assert_eq!(
        &output,
        b"100 = 100 %\n? ~= 12 %\ntrue\n200 = 100 %\n? ~= 12 %\nfalse\n"
    );
    Ok(())
}

#[test]
fn percents_1() -> Result<()> {
    let questions: Vec<Box<dyn Question>> = vec![
        Box::new(Percent {
            full: 123.0,
            percent: 12.0,
            precision: 1,
        }),
        Box::new(Percent {
            full: 123.0,
            percent: 12.0,
            precision: 1,
        }),
    ];
    let mut input = "14.7\n14\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run(&questions, &PipeMod::Skip, &mut input, &mut output)?;
    assert_eq!(
        &output,
        b"123 = 100 %\n? ~= 12 %\ntrue\n123 = 100 %\n? ~= 12 %\nfalse\n"
    );
    Ok(())
}

#[test]
fn abstract_seq_missing_0() -> Result<()> {
    let item1 = SeqItem {
        form: Form::Star,
        size: Size::Small,
    };
    let item2 = SeqItem {
        form: Form::Ampersand,
        size: Size::Big,
    };
    let item3 = SeqItem {
        form: Form::Cover,
        size: Size::Medium,
    };
    let questions: Vec<Box<dyn Question>> = vec![
        Box::new(Missing {
            items: vec![item1, item2, item3],
            options: vec![item1, item2],
            solution: item2,
        }),
        Box::new(Missing {
            items: vec![item1, item2, item3],
            options: vec![item3, item2],
            solution: item3,
        }),
    ];
    let mut input = "2\n2\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run(&questions, &PipeMod::Skip, &mut input, &mut output)?;
    assert_eq!(
        &output,
        b"* ? ^^\n1. * 2. &&&\ntrue\n* &&& ?\n1. ^^ 2. &&&\nfalse\n"
    );
    Ok(())
}

#[test]
fn abstract_seq_missing_1() -> Result<()> {
    let item1 = SeqItem {
        form: Form::Star,
        size: Size::Small,
    };
    let item2 = SeqItem {
        form: Form::Ampersand,
        size: Size::Big,
    };
    let item3 = SeqItem {
        form: Form::Cover,
        size: Size::Medium,
    };
    let questions: Vec<Box<dyn Question>> = vec![
        Box::new(Missing {
            items: all_combinations(),
            options: vec![item1, item2, item3],
            solution: item3,
        }),
        Box::new(Missing {
            items: all_combinations(),
            options: vec![item1, item2, item3],
            solution: item1,
        }),
    ];
    let mut input = "1\n1\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run(&questions, &PipeMod::Skip, &mut input, &mut output)?;
    assert_eq!(
        &output,
        b"* ** *** & && &&& ^ ? ^^^\n1. * 2. &&& 3. ^^\nfalse\n? ** *** & && &&& ^ ^^ ^^^\n1. * 2. &&& 3. ^^\ntrue\n"
    );
    Ok(())
}

#[test]
fn mod_until_right_0() -> Result<()> {
    let questions: Vec<Box<dyn Question>> = vec![Box::new(Sum { a: 1, b: 1 })];
    let mut input = "12\n2\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run(&questions, &PipeMod::UntilRight, &mut input, &mut output)?;
    assert_eq!(&output, b"1 + 1 = ?\nfalse\n1 + 1 = ?\ntrue\n");
    Ok(())
}

#[test]
fn mod_until_right_1() -> Result<()> {
    let questions: Vec<Box<dyn Question>> =
        vec![Box::new(Sum { a: 1, b: 1 }), Box::new(Sum { a: 2, b: 3 })];
    let mut input = "2\n2\n5\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run(&questions, &PipeMod::UntilRight, &mut input, &mut output)?;
    assert_eq!(&output, b"1 + 1 = ?\ntrue\n2 + 3 = ?\nfalse\n2 + 3 = ?\ntrue\n");
    Ok(())
}
