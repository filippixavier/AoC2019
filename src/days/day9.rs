use std::error::Error;
use std::fs;
use std::path::Path;

use super::intcode::Intcode;

fn prepare_file(input: String) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|x| (x.trim().parse::<i64>().unwrap_or(0)))
        .collect::<Vec<_>>()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let memory = prepare_file(fs::read_to_string(Path::new("./data/day9.txt"))?);
    let intmachine = Intcode::new(memory).add_input(1).run();
    println!("{}, {:?}", intmachine.output, intmachine.status);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
