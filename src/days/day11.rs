use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

use super::intcode::{CompStatus, Intcode};

type Coordinate = (i32, i32);

struct Robot {
    coordinate: Coordinate,
    brain: Intcode,
    directions: Vec<Coordinate>,
    map: HashMap<Coordinate, usize>,
}

impl Robot {
    pub fn new(memory: Vec<i64>) -> Self {
        Robot {
            coordinate: (0, 0),
            brain: Intcode::new(memory),
            directions: vec![(0, -1), (1, 0), (0, 1), (-1, 0)],
            map: HashMap::new(),
        }
    }
    pub fn launch_star1(mut self) -> Self {
        while self.brain.status != CompStatus::Halted {
            let current_color = if let Some(color) = self.map.get(&self.coordinate) {
                *color as i64
            } else {
                0
            };
            self.brain = self.brain.add_input(current_color).run();
            let (new_color, moving) = (
                self.brain.outputs.pop().unwrap(),
                self.brain.outputs.pop().unwrap(),
            );
            self.map.insert(self.coordinate, new_color as usize);

            let temp = self.directions.iter().cycle();

            self.directions = if moving == 0 {
                temp.skip(3).take(4).cloned().collect()
            } else {
                temp.skip(1).take(4).cloned().collect()
            };

            self.coordinate.0 += self.directions[0].0;
            self.coordinate.1 += self.directions[0].1;
        }
        self
    }
}

fn prepare_file(input: String) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|x| (x.trim().parse::<i64>().unwrap_or(0)))
        .collect::<Vec<_>>()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let memory = prepare_file(fs::read_to_string(Path::new("./data/day11.txt"))?);
    let robot = Robot::new(memory).launch_star1();
    println!("Number of individual panel painted: {}", robot.map.len());
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
