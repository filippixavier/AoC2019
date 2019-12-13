use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

use super::intcode::{CompStatus, Intcode};

fn prepare_file(input: String) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|x| (x.trim().parse::<i64>().unwrap_or(0)))
        .collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let memory = prepare_file(fs::read_to_string(Path::new("./data/day13.txt"))?);
    let int_machine = Intcode::new(memory).run();
    let mut map: HashMap<(i64, i64), i64> = HashMap::new();

    let mut outputs = int_machine.outputs.clone();
    outputs.reverse();

    for chunk in outputs.into_iter().chunks(3).into_iter() {
        let values = chunk.collect::<Vec<i64>>();
        if values.len() == 3 {
            map.insert((values[0], values[1]), values[2]);
        }
    }

    let tiles = map
        .values()
        .fold(0, |acc, val| if *val == 2 { acc + 1 } else { acc });
    println!("Blocks? {}", tiles);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut memory = prepare_file(fs::read_to_string(Path::new("./data/day13.txt"))?);
    memory[0] = 2;
    let mut int_machine = Intcode::new(memory).run();

    // init phase
    let mut blocks: HashSet<Position> = HashSet::new();
    let mut ball = Position { x: 0, y: 0 };
    let mut paddle = Position { x: 0, y: 0 };
    let mut score = 0;

    let mut outputs = int_machine.get_outputs();
    outputs.reverse();

    for chunk in outputs.into_iter().chunks(3).into_iter() {
        let values = chunk.collect::<Vec<i64>>();
        if values.len() == 3 {
            match (values[0], values[1], values[2]) {
                (-1, 0, x) => {
                    score = x;
                }
                (x, y, 2) => {
                    blocks.insert(Position { x, y });
                }
                (x, y, 3) => {
                    paddle.x = x;
                    paddle.y = y;
                }
                (x, y, 4) => {
                    ball.x = x;
                    ball.y = y;
                }
                _ => {}
            }
        }
    }

    //Update
    while int_machine.status != CompStatus::Halted && !blocks.is_empty() {
        int_machine = int_machine
            .add_input(if ball.x < paddle.x {
                -1
            } else if ball.x > paddle.x {
                1
            } else {
                0
            })
            .run();
        let mut outputs = int_machine.get_outputs();
        outputs.reverse();

        for chunk in outputs.into_iter().chunks(3).into_iter() {
            let values = chunk.collect::<Vec<i64>>();
            if values.len() == 3 {
                match (values[0], values[1], values[2]) {
                    (-1, 0, x) => {
                        score = x;
                    }
                    (x, y, 3) => {
                        paddle.x = x;
                        paddle.y = y;
                    }
                    (x, y, 4) => {
                        ball.x = x;
                        ball.y = y;
                    }
                    (x, y, value) => {
                        let posi = Position { x, y };
                        if blocks.contains(&posi) && value == 0 {
                            blocks.remove(&posi);
                        }
                    }
                }
            }
        }
    }

    println!("Total score is: {}", score);

    Ok(())
}
