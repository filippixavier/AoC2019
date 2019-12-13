use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;
use std::collections::HashMap;

use super::intcode::Intcode;

fn prepare_file(input: String) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|x| (x.trim().parse::<i64>().unwrap_or(0)))
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let memory = prepare_file(fs::read_to_string(Path::new("./data/day13.txt"))?);
    let int_machine = Intcode::new(memory).run();
    let mut map: HashMap<(i64, i64), i64> = HashMap::new();

    let mut outputs = int_machine.outputs.clone();
    outputs.reverse();

    for chunk in outputs.into_iter().chunks(3).into_iter() {
        let values = chunk.collect::<Vec<i64>>();
        if values.len() == 3 {
            map.insert((values[0], values[1]), values[2]);
        }
    }

    let tiles = map
        .values()
        .fold(0, |acc, val| if *val == 2 { acc + 1 } else { acc });
    println!("Blocks? {}", tiles);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
