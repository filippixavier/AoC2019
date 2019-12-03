use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashSet;

type Coordinate = (i32, i32);

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    // We have to use two statement because of the lifetime ellision and "temporary value dropped while borrowed"
    // The String created won't last long enougth if I use it without storing it
    let wires_path = fs::read_to_string(Path::new("./data/day3.txt"))?;
    
    let wires_path = wires_path.trim()
        .split('\n')
        .map(|elem| elem.split(','))
        .collect::<Vec<_>>();

    let mut wiring: HashSet<Coordinate> = HashSet::new();
    let mut smallest_dist: Option<i32> = None;

    for path in wires_path {
        let mut coord = (0, 0);
        for movement in path {
            let length = movement[1..].parse::<i32>().unwrap_or(0);
            let mut lateral = true;
            let mut positive = true;

            match movement.chars().next().unwrap() {
                'U' => {
                    lateral = false;
                }
                'D' => {
                    lateral = false;
                    positive = false;
                }
                'L' => {
                    positive = false
                }
                'R' => {}
                a => {
                    println!("{}", a);
                    panic!("Should NOT happen")
                }
            }

            for _ in 0..length {
                let offset = if positive {1} else {-1};
                let temp_coord = if lateral {(coord.0 + offset, coord.1)} else {(coord.0, coord.1 + offset)};
                coord = temp_coord;
                if !wiring.insert(temp_coord) {
                    let curr_dist = coord.0.abs() + coord.1.abs();
                    // get man distance
                    smallest_dist = if let Some(dist) = smallest_dist {
                        if dist < curr_dist {
                            Some(dist)
                        } else {
                            Some(curr_dist)
                        }
                    } else {
                        Some(curr_dist)
                    }
                }
              }
        }
    }

    println!("Smallest dist is: {}", smallest_dist.unwrap_or(0));

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}