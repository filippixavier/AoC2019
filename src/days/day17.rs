use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;

use super::intcode;

type Coordinate = (usize, usize);

#[derive(Debug, PartialEq)]
struct PathWay {
    junctions: isize,
}

impl PathWay {
    pub fn new() -> Self {
        PathWay { junctions: 0 }
    }
}

#[derive(Debug)]
enum Tile {
    Empty,
    Scaffold(PathWay),
    Robot,
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    use Tile::*;
    let mut robot =
        intcode::Intcode::new_with_path(fs::read_to_string(Path::new("./data/day17.txt"))?);

    let mut map: HashMap<Coordinate, Tile> = HashMap::new();
    let mut alignement = 0;

    robot = robot.run();

    let mut output = robot.get_outputs();
    output.reverse(); // Because (How could I forgot that...) inputs and outputs are store in a fifo order :D

    let ascii_map = output
        .into_iter()
        .map(|x| (x as u8) as char)
        .collect::<String>();
    let ascii_map = ascii_map.trim();

    for (line_no, line) in ascii_map.split('\n').enumerate() {
        for (col_no, character) in line.chars().enumerate() {
            let coordinate = (col_no, line_no);
            let tile = match character {
                '.' => Empty,
                '#' => {
                    let mut pathway = PathWay::new();
                    if coordinate.0 > 0 {
                        let other_coordinate = (coordinate.0 - 1, coordinate.1);
                        if let Scaffold(other_path) =
                            map.get_mut(&other_coordinate).unwrap_or(&mut Empty)
                        {
                            other_path.junctions += 1;
                            pathway.junctions += 1;

                            if other_path.junctions == 3 {
                                alignement += other_coordinate.0 * other_coordinate.1;
                            }
                        }
                    }
                    if coordinate.1 > 0 {
                        let other_coordinate = (coordinate.0, coordinate.1 - 1);
                        if let Scaffold(other_path) =
                            map.get_mut(&other_coordinate).unwrap_or(&mut Empty)
                        {
                            other_path.junctions += 1;
                            pathway.junctions += 1;

                            if other_path.junctions == 3 {
                                alignement += other_coordinate.0 * other_coordinate.1;
                            }
                        }
                    }

                    Scaffold(pathway)
                }
                _ => Robot,
            };
            map.insert(coordinate, tile);
        }
    }

    println!("Sum of alignement parameters: {}", alignement);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
