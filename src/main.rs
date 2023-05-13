mod abstract_sequence;
mod arithmetic;
mod input;
mod percentage;
mod stats;
mod store;
mod task;

mod tasks_pipe;
use core::time;
use std::{
    collections::HashSet,
    fmt::format,
    time::{Duration, Instant},
};

use crate::stats::calculate_average_time_millis;
use abstract_sequence::{all_combinations, Missing, SeqItem};
use anyhow::{anyhow, Ok, Result};
use arithmetic::{Div, Mod, Mul, Sub, Sum};
use clap::Parser;
use percentage::Percent;
use rand::{seq::SliceRandom, Rng};
use rusqlite::Connection;
use stats::{calculate_total_pos_neg, StatsConfig};
use store::stats as store_stats;
use task::Question;
use tasks_pipe::{run, run_with_stats, run_without_steps, PipeMod};
use uuid::Uuid;

#[derive(Parser)]
struct Args {
    count: u32,
    exersise: String,
    pipe_mod: Option<String>,
    stats_config: Option<String>,
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
    // todo handle incorrect stats config options
    let stats_configs = match args.stats_config {
        Some(opts) => Some(parse_config_stat_options(opts)),
        None => None,
    };

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
    let stats_config = stats_configs.map(|opts| StatsConfig {
        time: opts.contains("time"),
        percentage: opts.contains("percentage"),
    });

    match stats_config {
        Some(value) => {
            let stats = run_with_stats(
                &questions,
                &pipe_mod,
                &mut std::io::stdin().lock(),
                &mut std::io::stdout(),
                &value,
            )?;
            if stats.times_millis.is_some() {
                let average = calculate_average_time_millis(stats.times_millis.unwrap());
                println!("Average time: {} secs", average / 1000);
            }
            if stats.pos_negs.is_some() {
                let pos_neg = calculate_total_pos_neg(stats.pos_negs.unwrap());
                println!(
                    "Rate: {} / {}",
                    pos_neg.positive,
                    pos_neg.positive + pos_neg.negative
                );
            }
            let connection = Connection::open("data/stats.db")?;

            store_stats::create_table_if_not_exist(&connection)?;
            let stats = store_stats::Stats {
                id: Uuid::new_v4().to_string(),
                question_type: String::from("sum"),
                formatted_body: String::from("1 + 1 = ?"),
                is_answer_right: false,
                time_millis: 1200,
                created_at_millis: 12121212,
            };
            store_stats::insert_or_replace(&connection, stats)?;
            let result = store_stats::select_all(&connection)?;
            let _ = connection.close();
            print_stats(&result);
        }
        None => {
            run_without_steps(
                &questions,
                &pipe_mod,
                &mut std::io::stdin().lock(),
                &mut std::io::stdout(),
            )?;
        }
    }

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

fn parse_config_stat_options(opts: String) -> HashSet<String> {
    let opts = opts.split(" ");
    let mut vector: Vec<String> = Vec::new();
    for opt in opts {
        vector.push(String::from(opt.trim()));
    }
    vector.into_iter().collect()
}

fn print_stats(items: &Vec<store_stats::Stats>) {
    for item in items {
        println!("{:?}", item);
    }
}
