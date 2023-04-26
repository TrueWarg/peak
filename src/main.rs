mod abstract_sequence;
mod arithmetic;
mod percentage;
mod task;
mod tasks_pipe;
use std::{collections::HashSet, fmt::format};

use abstract_sequence::{all_combinations, Missing, SeqItem};
use anyhow::{anyhow, Ok, Result};
use arithmetic::{Div, Mod, Mul, Sub, Sum};
use clap::Parser;
use percentage::Percent;
use rand::{seq::SliceRandom, Rng};
use task::Question;
use tasks_pipe::{run, PipeMod};

#[derive(Parser)]
struct Args {
    count: u32,
    exersise: String,
    pipe_mod: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut rng = rand::thread_rng();
    let mut questions: Vec<Box<dyn Question>> = Vec::new();
    let types: HashSet<&str> = vec!["sum", "sub", "mul", "div", "mod", "percent", "missing"]
        .into_iter()
        .collect();
    let modes: HashSet<&str> = vec!["right", "skip"].into_iter().collect();

    let typ = args.exersise.as_str();
    let pipe_mod = match args.pipe_mod {
        Some(value) => value,
        None => String::from("skip"),
    };
    if !types.contains(typ) {
        let message = format!("unknown type `{}`", &args.exersise);
        return Err(anyhow!(message));
    }
    if !modes.contains(pipe_mod.as_str()) {
        let message = format!("unknown mod `{}`", &pipe_mod);
        return Err(anyhow!(message));
    }
    for _ in 1..&args.count + 1 {
        if typ == "sum" {
            let value = Sum {
                a: rng.gen_range(0..100),
                b: rng.gen_range(0..100),
            };
            questions.push(Box::new(value));
        }
        if typ == "sub" {
            let value = Sub {
                a: rng.gen_range(0..100),
                b: rng.gen_range(0..100),
            };
            questions.push(Box::new(value));
        }
        if typ == "mul" {
            let value = Mul {
                a: rng.gen_range(0..25),
                b: rng.gen_range(0..25),
            };
            questions.push(Box::new(value));
        }
        if typ == "div" {
            let value = Div {
                a: rng.gen_range(1..20),
                b: rng.gen_range(1..10),
            };
            questions.push(Box::new(value));
        }
        if typ == "mod" {
            let value = Mod {
                a: rng.gen_range(1..20),
                b: rng.gen_range(1..10),
            };
            questions.push(Box::new(value));
        }
        if typ == "percent" {
            let value = Percent {
                full: rng.gen_range(1..1000) as f64,
                percent: rng.gen_range(1..100) as f64,
                precision: 1,
            };
            questions.push(Box::new(value));
        }
        if typ == "missing" {
            let value = abstract_seq_missing();
            questions.push(Box::new(value));
        }
    }
    let pipe_mod = match pipe_mod.as_str() {
        "skip" => PipeMod::Skip,
        "right" => PipeMod::UntilRight,
        _ => PipeMod::Skip,
    };
    run(
        &questions,
        &pipe_mod,
        &mut std::io::stdin().lock(),
        &mut std::io::stdout(),
    )?;
    Ok(())
}

fn abstract_seq_missing() -> Missing {
    let mut rng = rand::thread_rng();
    let mut items = all_combinations();
    items.shuffle(&mut rng);
    let length = items.len();
    let solution = items[rng.gen_range(0..length)];
    let mut options: Vec<SeqItem> = vec![];
    let options_count = 4;
    let right_position = rng.gen_range(0..options_count);
    loop {
        if options.len() == right_position {
            options.push(solution);
        }
        if options.len() == options_count {
            break;
        }
        let candidate = items[rng.gen_range(0..length)];
        if !options.contains(&candidate) {
            options.push(candidate);
        }
    }
    return Missing {
        items,
        options,
        solution,
    };
}

// 3. combinations
// 4. Regimes: skip, until right
// 5. Statistic.

// peak -- help
// peak start -> default
// peak stat [clear] [delete id]
// peak settings
