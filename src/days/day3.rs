use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;
use std::collections::HashSet;

type Coordinate = (i32, i32);

fn prepare_input(input: String) -> Vec<Vec<String>> {
    // We have to use two statement because of the lifetime ellision and "temporary value dropped while borrowed"
    // The String created won't last long enougth if I use it without storing it
    input
        .trim()
        .split('\n')
        .map(|elem| {
            elem.split(',')
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<_>>()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let wires_path = prepare_input(fs::read_to_string(Path::new("./data/day3.txt"))?);
    let mut wiring: HashSet<Coordinate> = HashSet::new();
    let mut smallest_dist: Option<i32> = None;

    for (index, path) in wires_path.iter().enumerate() {
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
                'L' => positive = false,
                'R' => {}
                a => {
                    println!("{}", a);
                    panic!("Should NOT happen")
                }
            }

            for _ in 0..length {
                let offset = if positive { 1 } else { -1 };
                let temp_coord = if lateral {
                    (coord.0 + offset, coord.1)
                } else {
                    (coord.0, coord.1 + offset)
                };
                coord = temp_coord;
                if !wiring.insert(temp_coord) && index == 1 {
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
    let wires_path = prepare_input(fs::read_to_string(Path::new("./data/day3.txt"))?);
    let mut wiring: HashMap<Coordinate, (usize, usize)> = HashMap::new();
    let mut smallest_timing: Option<usize> = None;

    for (index, path) in wires_path.iter().enumerate() {
        let flag = index + 1;
        let mut coord = (0, 0);
        let mut steps = 0;
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
                'L' => positive = false,
                'R' => {}
                a => {
                    println!("{}", a);
                    panic!("Should NOT happen")
                }
            }

            for _ in 0..length {
                steps += 1;
                let offset = if positive { 1 } else { -1 };
                coord = if lateral {
                    (coord.0 + offset, coord.1)
                } else {
                    (coord.0, coord.1 + offset)
                };
                if let Some(crossroad) = wiring.get(&coord) {
                    if flag & crossroad.1 != flag {
                        let total = crossroad.0 + steps;
                        smallest_timing = if let Some(timing) = smallest_timing {
                            Some(std::cmp::min(total, timing))
                        } else {
                            Some(total)
                        }
                    }
                } else {
                    wiring.insert(coord, (steps, flag));
                }
            }
        }
    }

    println!("Smallest timing: {}", smallest_timing.unwrap_or(0));

    Ok(())
}

//2714 too low
