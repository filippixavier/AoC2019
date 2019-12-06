use std::error::Error;
use std::fs;
use std::path::Path;

use super::intcode::Intcode;

fn prepare_file(input: String) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|x| (x.trim().parse::<i32>().unwrap_or(0)))
        .collect::<Vec<_>>()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mem = prepare_file(fs::read_to_string(Path::new("./data/day5.txt"))?);
    let mut int_machine = Intcode::new(mem, 1);

    while int_machine.next_op() {}

    println!("The final output is: {}", int_machine.output);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let mem = prepare_file(fs::read_to_string(Path::new("./data/day5.txt"))?);
    let mut int_machine = Intcode::new(mem, 5);

    while int_machine.next_op() {}

    println!("The final output is: {}", int_machine.output);

    Ok(())
}