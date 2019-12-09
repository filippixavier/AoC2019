use itertools::Itertools;
use std::error::Error;
use std::fs;
use std::path::Path;

use super::intcode::{CompStatus, Intcode};

fn prepare_file(input: String) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|x| (x.trim().parse::<i64>().unwrap_or(0)))
        .collect::<Vec<_>>()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let memory = prepare_file(fs::read_to_string(Path::new("./data/day7.txt"))?);

    let permutations_it = (0..5).permutations(5);

    let mut max_thruster: Option<i64> = None;
    let mut permutation_str: String = String::from("None");

    for permutation in permutations_it {
        let current_permutation_str = permutation.iter().map(|elem| elem.to_string()).collect();

        let first_amp = Intcode::new(memory.clone())
            .add_input(permutation[0])
            .run()
            .add_input(0)
            .run();
        let second_amp = Intcode::new(memory.clone())
            .add_input(permutation[1])
            .run()
            .add_input(first_amp.output)
            .run();
        let third_amp = Intcode::new(memory.clone())
            .add_input(permutation[2])
            .run()
            .add_input(second_amp.output)
            .run();
        let fourth_amp = Intcode::new(memory.clone())
            .add_input(permutation[3])
            .run()
            .add_input(third_amp.output)
            .run();
        let fifth_amp = Intcode::new(memory.clone())
            .add_input(permutation[4])
            .run()
            .add_input(fourth_amp.output)
            .run();

        let thruster = fifth_amp.output;

        max_thruster = if let Some(current_max) = max_thruster {
            if current_max < thruster {
                permutation_str = current_permutation_str;
                Some(thruster)
            } else {
                Some(current_max)
            }
        } else {
            permutation_str = current_permutation_str;
            Some(thruster)
        }
    }

    println!(
        "Max thruster: {} for permutation: {}",
        max_thruster.unwrap_or(-1),
        permutation_str
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let memory = prepare_file(fs::read_to_string(Path::new("./data/day7.txt"))?);

    let permutations_it = (5..10).permutations(5);

    let mut max_thruster: Option<i64> = None;
    let mut permutation_str: String = String::from("None");

    for permutation in permutations_it {
        let current_permutation_str = permutation.iter().map(|elem| elem.to_string()).collect();

        let mut first_amp = Intcode::new(memory.clone()).add_input(permutation[0]).run();

        let mut second_amp = Intcode::new(memory.clone()).add_input(permutation[1]).run();
        let mut third_amp = Intcode::new(memory.clone()).add_input(permutation[2]).run();

        let mut fourth_amp = Intcode::new(memory.clone()).add_input(permutation[3]).run();

        let mut fifth_amp = Intcode::new(memory.clone()).add_input(permutation[4]).run();

        let mut input = 0;

        while fifth_amp.status != CompStatus::Halted {
            first_amp = first_amp.add_input(input).run();
            second_amp = second_amp.add_input(first_amp.output).run();
            third_amp = third_amp.add_input(second_amp.output).run();
            fourth_amp = fourth_amp.add_input(third_amp.output).run();
            fifth_amp = fifth_amp.add_input(fourth_amp.output).run();

            input = fifth_amp.output;
        }

        max_thruster = if let Some(current_max) = max_thruster {
            if current_max < input {
                permutation_str = current_permutation_str;
                Some(input)
            } else {
                Some(current_max)
            }
        } else {
            permutation_str = current_permutation_str;
            Some(input)
        }
    }

    println!(
        "Max thruster: {} for permutation: {}",
        max_thruster.unwrap_or(-1),
        permutation_str
    );

    Ok(())
}
