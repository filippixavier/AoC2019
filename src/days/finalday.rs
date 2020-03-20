use std::error::Error;
use std::fs;
use std::path::Path;

use std::io::{self};

use super::intcode;

// Yeah, I've basically played the game instead of trying to solve it :/
pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut drone =
        intcode::Intcode::new_with_path(fs::read_to_string(Path::new("./data/finalday.txt"))?);

    loop {
        drone = drone.run();

        let mut output = drone.get_outputs();
        output.reverse(); 

        let output = output
            .into_iter()
            .map(|x| (x as u8) as char)
            .collect::<String>();
        let output = output.trim();

        println!("{}", output);

        match drone.status {
            intcode::CompStatus::Halted => {
                println!("Drone halted!");
                break;
            }
            intcode::CompStatus::Error => {
                panic!("Something went wrong?");
            }
            _ => {}
        }

        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                let buffer = buffer.trim();
                match buffer {
                    "exit" => {
                        println!("Exiting adventure game");
                        break;
                    }
                    "clear" => {
                        print!("\x1B[2J");
                        continue;
                    }
                    _ => {}
                }
                // Must use trim_end or the line ending mess up with the parsing function
                let mut inputs = buffer
                    .chars()
                    .map(|x| (x as u8) as i64)
                    .collect::<Vec<i64>>();
                inputs.push(10);
                drone.set_inputs(&inputs);
            }
            Err(error) => panic!("Error: {}, input went wrong", error),
        }
    }

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
