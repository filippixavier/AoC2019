use std::error::Error;
use std::fs;
use std::path::Path;

use super::intcode;

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut count = 0;

    let original =
        intcode::Intcode::new_with_path(fs::read_to_string(Path::new("./data/day19.txt"))?);

    for x in 0..50 {
        for y in 0..50 {
            let mut drone = original.clone();
            drone = drone.add_input(x).add_input(y).run();
            let is_tracted = drone.get_outputs()[0];

            if is_tracted == 1 {
                count += 1;
            }
        }
    }

    println!("Tracted in {} zones", count);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    const OFFSET: i64 = 99;

    let mut y = OFFSET;
    let mut min_x = 0;

    let original =
        intcode::Intcode::new_with_path(fs::read_to_string(Path::new("./data/day19.txt"))?);

    'outer: loop {
        for x in min_x.. {
            let mut drone = original.clone();
            drone = drone.add_input(x).add_input(y).run();

            let mut is_tracted = drone.get_outputs()[0];

            if is_tracted == 1 {
                let mut drone = original.clone();
                drone = drone.add_input(x + OFFSET).add_input(y - OFFSET).run();
                is_tracted = drone.get_outputs()[0];
            } else {
                continue;
            }

            if is_tracted == 1 {
                println!("Result is: {}", x * 10_000 + (y - OFFSET));
                break 'outer;
            } else {
                y += 1;
                min_x = std::cmp::max(0, x - 5);
                break;
            }
        }
    }

    Ok(())
}
