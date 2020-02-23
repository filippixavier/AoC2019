use std::error::Error;
use std::fs;
use std::path::Path;

use super::intcode;

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut jumping_drone =
        intcode::Intcode::new_with_path(fs::read_to_string(Path::new("./data/day21.txt"))?);
    let program = "OR A T\nAND B T\nAND C T\nNOT T J\nAND D J\nWALK\n";

    let jmp_prog = program
        .chars()
        .map(|x| (x as u8) as i64)
        .collect::<Vec<i64>>();

    jumping_drone = jumping_drone.run();

    jumping_drone.get_outputs();

    for i in jmp_prog {
        jumping_drone = jumping_drone.add_input(i);
    }

    jumping_drone = jumping_drone.run();

    let mut output = jumping_drone.get_outputs();

    output.reverse();

    let text = output.iter().fold(String::new(), |mut acc, x| {
        if *x < 256 {
            let character = (*x as u8) as char;
            acc.push(character);
        } else {
            return format!("{}{}", acc, x);
        }

        acc
    });

    println!("Result:\n{}", text);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
