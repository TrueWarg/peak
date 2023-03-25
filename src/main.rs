mod arithmetic;
mod task;
use std::io;

use anyhow::{Context, Ok, Result};
use arithmetic::Plus;
use clap::Parser;
use rand::Rng;
use task::Question;

use crate::task::default_check;

#[derive(Parser)]
struct Args {
    count: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut rng = rand::thread_rng();
    for i in 0..args.count {
        println!("{} / {}", i + 1, args.count);
        let plus = Plus {
            a: rng.gen_range(0..100),
            b: rng.gen_range(0..100),
        };
        let body = plus.body();
        println!("{} = ?", body);
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let answer: i32 = line.trim().parse().expect("Input not an integer");
        let result = default_check::<Plus>(plus.solution(), answer);
        let message = if result { "Right" } else { "Wrong" };
        println!("{}", message);
    }
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
