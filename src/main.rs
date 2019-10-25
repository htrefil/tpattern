#![feature(process_exitcode_placeholder)]
mod model;
mod words;

use model::{Generator, Model};
use rand::rngs::OsRng;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::process::ExitCode;
use words::Words;

fn usage() {
    println!("Usage: {} <path> <count>", env!("CARGO_PKG_NAME"));
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let path = match args.next() {
        Some(path) => path,
        None => {
            usage();
            return ExitCode::FAILURE;
        }
    };

    let count = match args.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(count) => count,
        None => {
            usage();
            return ExitCode::FAILURE;
        }
    };

    let data = match fs::read_to_string(&path) {
        Ok(data) => data,
        Err(err) => {
            println!("Error reading {}: {}", path, err);
            return ExitCode::FAILURE;
        }
    };

    let model = Model::new(Words::new(&data));
    let result = Generator::new(&model, &mut OsRng)
        .map(|f| format!("{} ", f))
        .take(count)
        .collect::<String>();

    println!("{}", result);

    ExitCode::SUCCESS
}
