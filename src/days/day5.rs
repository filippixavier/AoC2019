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
    let int_machine = Intcode::new(mem).add_input(1).run();
    println!("The final output is: {}", int_machine.output);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let mem = prepare_file(fs::read_to_string(Path::new("./data/day5.txt"))?);
    let int_machine = Intcode::new(mem).add_input(5).run();

    println!("The final output is: {}", int_machine.output);

    Ok(())
}
