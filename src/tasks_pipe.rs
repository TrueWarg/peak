use crate::{
    abstract_sequence::{all_combinations, Form, Missing, SeqItem, Size},
    arithmetic::{Div, Mod, Mul, Sub, Sum},
    input::{self, DefferedInput},
    percentage::Percent,
    stats::{CollectedStats, StatsConfig},
    task::Question,
};
use anyhow::{Ok, Result};
use std::{
    io::{BufRead, Read, Write},
    iter::Skip,
    thread,
    time::{Duration, Instant},
};

#[derive(Debug, PartialEq, Eq)]
pub enum PipeMod {
    UntilRight,
    Skip,
}

pub fn run_without_steps<Q: Question + ?Sized>(
    questions: &Vec<Box<Q>>,
    pipe_mod: &PipeMod,
    mut reader: impl BufRead,
    mut writer: impl Write,
) -> Result<()> {
    run(questions, pipe_mod, reader, writer, |_| (), |_, _| ())
}

pub fn run_with_stats<Q: Question + ?Sized>(
    questions: &Vec<Box<Q>>,
    pipe_mod: &PipeMod,
    mut reader: impl BufRead,
    mut writer: impl Write,
    stats_config: &StatsConfig,
) -> Result<CollectedStats> {
    let instant = Instant::now();
    let mut times: Vec<u64> = vec![];
    let mut pos_negs: Vec<bool> = vec![];

    run(
        &questions,
        &pipe_mod,
        reader,
        writer,
        |_| (),
        |_, answer| {
            if answer && pipe_mod == &PipeMod::UntilRight || pipe_mod == &PipeMod::Skip {
                if stats_config.time {
                    times.push(instant.elapsed().as_secs())
                }
                if stats_config.percentage {
                    pos_negs.push(answer)
                }
            }
        },
    )?;
    let times = if stats_config.time { Some(times) } else { None };
    let pos_negs = if stats_config.percentage {
        Some(pos_negs)
    } else {
        None
    };
    Ok(CollectedStats {
        times_secs: times,
        pos_negs: pos_negs,
    })
}

// todo: move reader and writer to step functiobs?
pub fn run<Q: Question + ?Sized, FStart: FnMut(&Q) -> (), FEnd: FnMut(&Q, bool) -> ()>(
    questions: &Vec<Box<Q>>,
    pipe_mod: &PipeMod,
    mut reader: impl BufRead,
    mut writer: impl Write,
    mut on_step_start: FStart,
    mut on_step_end: FEnd,
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
        on_step_start(&question);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let result = question.check(&line);
        match result {
            Result::Ok(correct) => {
                on_step_end(&question, correct);
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
    run_without_steps(&questions, &PipeMod::Skip, &mut input, &mut output)?;
    assert_eq!(&output, b"1 + 1 = ?\ntrue\n1 - 1 = ?\ntrue\n");
    Ok(())
}

#[test]
fn sum_and_sub_1() -> Result<()> {
    let questions: Vec<Box<dyn Question>> =
        vec![Box::new(Sum { a: 1, b: 1 }), Box::new(Sub { a: 1, b: 1 })];
    let mut input = "2\n2\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run_without_steps(&questions, &PipeMod::Skip, &mut input, &mut output)?;
    assert_eq!(&output, b"1 + 1 = ?\ntrue\n1 - 1 = ?\nfalse\n");
    Ok(())
}

#[test]
fn sum_fail_on_non_digit() -> Result<()> {
    let questions: Vec<Box<dyn Question>> = vec![Box::new(Sum { a: 1, b: 1 })];
    let mut input = "kek\n2\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run_without_steps(&questions, &PipeMod::Skip, &mut input, &mut output)?;
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
    run_without_steps(&questions, &PipeMod::Skip, &mut input, &mut output)?;
    assert_eq!(&output, b"4 * 5 = ?\ntrue\n5 div 2 = ?\nfalse\n");
    Ok(())
}

#[test]
fn div_and_mod_0() -> Result<()> {
    let questions: Vec<Box<dyn Question>> =
        vec![Box::new(Div { a: 8, b: 4 }), Box::new(Mod { a: 5, b: 2 })];
    let mut input = "2\n1\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run_without_steps(&questions, &PipeMod::Skip, &mut input, &mut output)?;
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
    run_without_steps(&questions, &PipeMod::Skip, &mut input, &mut output)?;
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
    run_without_steps(&questions, &PipeMod::Skip, &mut input, &mut output)?;
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
    run_without_steps(&questions, &PipeMod::Skip, &mut input, &mut output)?;
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
    run_without_steps(&questions, &PipeMod::Skip, &mut input, &mut output)?;
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
    run_without_steps(&questions, &PipeMod::UntilRight, &mut input, &mut output)?;
    assert_eq!(&output, b"1 + 1 = ?\nfalse\n1 + 1 = ?\ntrue\n");
    Ok(())
}

#[test]
fn mod_until_right_1() -> Result<()> {
    let questions: Vec<Box<dyn Question>> =
        vec![Box::new(Sum { a: 1, b: 1 }), Box::new(Sum { a: 2, b: 3 })];
    let mut input = "2\n2\n5\n".as_bytes();
    let mut output: Vec<u8> = Vec::new();
    run_without_steps(&questions, &PipeMod::UntilRight, &mut input, &mut output)?;
    assert_eq!(
        &output,
        b"1 + 1 = ?\ntrue\n2 + 3 = ?\nfalse\n2 + 3 = ?\ntrue\n"
    );
    Ok(())
}

#[test]
fn mod_skip_with_stats_0() -> Result<()> {
    let questions: Vec<Box<dyn Question>> =
        vec![Box::new(Sum { a: 1, b: 1 }), Box::new(Sub { a: 1, b: 1 })];
    let mut input = DefferedInput {
        input: "2\n0\n".as_bytes(),
        delay_secs: 0,
    };
    let mut output: Vec<u8> = Vec::new();
    let stats_config = StatsConfig {
        time: true,
        percentage: false,
    };
    let expected_stats = CollectedStats {
        times_secs: Some(vec![0, 0]),
        pos_negs: None,
    };
    let stats = run_with_stats(
        &questions,
        &PipeMod::Skip,
        &mut input,
        &mut output,
        &stats_config,
    )?;
    assert_eq!(&output, b"1 + 1 = ?\ntrue\n1 - 1 = ?\ntrue\n");
    assert_eq!(stats, expected_stats);
    Ok(())
}

#[test]
fn mod_skip_with_stats_1() -> Result<()> {
    let questions: Vec<Box<dyn Question>> = vec![
        Box::new(Sum { a: 1, b: 1 }),
        Box::new(Sub { a: 1, b: 1 }),
        Box::new(Mul { a: 1, b: 2 }),
    ];
    let mut input = DefferedInput {
        input: "2\n0\n1\n".as_bytes(),
        delay_secs: 1,
    };
    let mut output: Vec<u8> = Vec::new();
    let stats_config = StatsConfig {
        time: true,
        percentage: true,
    };
    let expected_stats = CollectedStats {
        times_secs: Some(vec![1, 2, 3]),
        pos_negs: Some(vec![true, true, false]),
    };
    let stats = run_with_stats(
        &questions,
        &PipeMod::Skip,
        &mut input,
        &mut output,
        &stats_config,
    )?;
    assert_eq!(
        &output,
        b"1 + 1 = ?\ntrue\n1 - 1 = ?\ntrue\n1 * 2 = ?\nfalse\n"
    );
    assert_eq!(stats, expected_stats);
    Ok(())
}

#[test]
fn mod_skip_with_stats_2() -> Result<()> {
    let questions: Vec<Box<dyn Question>> =
        vec![Box::new(Sum { a: 1, b: 1 }), Box::new(Sub { a: 1, b: 1 })];
    let mut input = DefferedInput {
        input: "3\n0\n".as_bytes(),
        delay_secs: 1,
    };
    let mut output: Vec<u8> = Vec::new();
    let stats_config = StatsConfig {
        time: false,
        percentage: true,
    };
    let expected_stats = CollectedStats {
        times_secs: None,
        pos_negs: Some(vec![false, true]),
    };
    let stats = run_with_stats(
        &questions,
        &PipeMod::Skip,
        &mut input,
        &mut output,
        &stats_config,
    )?;
    assert_eq!(&output, b"1 + 1 = ?\nfalse\n1 - 1 = ?\ntrue\n");
    assert_eq!(stats, expected_stats);
    Ok(())
}

#[test]
fn mod_until_right_with_stats_2() -> Result<()> {
    let questions: Vec<Box<dyn Question>> =
        vec![Box::new(Sum { a: 1, b: 1 }), Box::new(Sum { a: 2, b: 3 })];
    let mut input = DefferedInput {
        input: "2\n2\n5\n".as_bytes(),
        delay_secs: 1,
    };
    let mut output: Vec<u8> = Vec::new();
    let stats_config = StatsConfig {
        time: true,
        percentage: true,
    };
    let expected_stats = CollectedStats {
        times_secs: Some(vec![1, 3]),
        pos_negs: Some(vec![true, true]),
    };
    let stats = run_with_stats(
        &questions,
        &PipeMod::UntilRight,
        &mut input,
        &mut output,
        &stats_config,
    )?;
    assert_eq!(
        &output,
        b"1 + 1 = ?\ntrue\n2 + 3 = ?\nfalse\n2 + 3 = ?\ntrue\n"
    );
    assert_eq!(stats, expected_stats);
    Ok(())
}
