use std::error::Error;
use std::fs;
use std::path::Path;

fn prepare_file(input: String) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|x| (x.trim().parse::<usize>().unwrap_or(0)))
        .collect::<Vec<_>>()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut input = prepare_file(fs::read_to_string(Path::new("./data/day2.txt"))?);
    input[1] = 12;
    input[2] = 2;
    let mut index = 0;
    loop {
        match input[index] {
            1 => {
                let store_index = input[index + 3];
                let first_index = input[index + 1];
                let second_index = input[index + 2];
                input[store_index] = input[first_index] + input[second_index];
            }
            2 => {
                let store_index = input[index + 3];
                let first_index = input[index + 1];
                let second_index = input[index + 2];
                input[store_index] = input[first_index] * input[second_index];
            }
            99 => {
                println!("Program halted, result at position 0: {}", input[0]);
                break;
            }
            _ => panic!("HALT AND CATCH FIRE"),
        }
        index += 4;
    }
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let memory = prepare_file(fs::read_to_string(Path::new("./data/day2.txt"))?);

    'outer: for i in 0..100 {
        for j in 0..100 {
            let mut input = memory.clone();
            input[1] = i;
            input[2] = j;

            let mut index = 0;
            loop {
                match input[index] {
                    1 => {
                        let store_index = input[index + 3];
                        let first_index = input[index + 1];
                        let second_index = input[index + 2];
                        input[store_index] = input[first_index] + input[second_index];
                    }
                    2 => {
                        let store_index = input[index + 3];
                        let first_index = input[index + 1];
                        let second_index = input[index + 2];
                        input[store_index] = input[first_index] * input[second_index];
                    }
                    99 => {
                        break;
                    }
                    _ => {
                        println!("HALT AND CATCH FIRE");
                        break;
                    }
                }
                index += 4;
            }

            if input[0] == 19_690_720 {
                println!(
                    "Noun: {}, verb: {}, answer: {}",
                    input[1],
                    input[2],
                    100 * input[1] + input[2]
                );
                break 'outer;
            }
        }
    }

    Ok(())
}
