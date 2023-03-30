mod arithmetic;
mod task;
mod tasks_pipe;
use std::{collections::HashSet};

use anyhow::{anyhow, Ok, Result};
use arithmetic::{Sub, Sum};
use clap::Parser;
use rand::Rng;
use task::Question;
use tasks_pipe::run;

#[derive(Parser)]
struct Args {
    count: u32,
    exersise: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut rng = rand::thread_rng();
    let mut questions: Vec<Box<dyn Question>> = Vec::new();
    let types: HashSet<&str> = vec!["sum", "sub"].into_iter().collect();
    let typ = args.exersise.as_str();
    if !types.contains(typ) {
        let message = format!("unknown type `{}`", &args.exersise);
        return Err(anyhow!(message));
    }
    for _ in 0..&args.count - 1 {
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
    }
    run(
        &questions,
        &mut std::io::stdin().lock(),
        &mut std::io::stdout(),
    )?;
    Ok(())
}

// plan
// 1. Arymphmetic (*, +, -, /)
//    int.
//    float
// 2. %
// 3. combinations
// 4. Regimes: skip, until right
// 5. Statistic.

// peak -- help
// peak start -> default
// peak stat [clear] [delete id]
// peak settings
