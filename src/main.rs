#![feature(process_exitcode_placeholder)]
mod model;
mod words;

use model::Model;
use rand::distributions::{Distribution, WeightedIndex};
use rand::rngs::OsRng;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::process::ExitCode;
use words::Words;

fn generate(model: &Model<&str>, count: usize) -> String {
    let map = model
        .nodes
        .iter()
        .map(|node| (node.item, node))
        .collect::<HashMap<_, _>>();
    let mut used = HashSet::new();
    let mut last = None;
    let mut result = String::new();

    for i in 0..count {
        loop {
            let node = last
                .take()
                .unwrap_or_else(|| &model.nodes[OsRng.gen_range(0, model.nodes.len())]);
            let word = if node.children.len() != 0 {
                let idx = WeightedIndex::new(&node.weights)
                    .unwrap()
                    .sample(&mut OsRng);
                node.children[idx]
            } else {
                node.item
            };

            if used.contains(&word) {
                used.clear();
                continue;
            }

            used.insert(word);

            if !result.is_empty() && word.chars().any(char::is_alphanumeric) {
                result += " ";
            }

            result += word;

            last = Some(map.get(&word).unwrap());
            break;
        }
    }

    result
}

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
    println!("{}", generate(&model, count));

    ExitCode::SUCCESS
}
