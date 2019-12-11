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
    pub fn paint(mut self) -> Self {
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

    pub fn to_img(&self) -> String {
        let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);
        for coord in self.map.keys() {
            min_x = std::cmp::min(min_x, coord.0);
            max_x = std::cmp::max(max_x, coord.0);
            min_y = std::cmp::min(min_y, coord.1);
            max_y = std::cmp::max(max_y, coord.1);
        }

        let x_length = max_x - min_x + 1;
        let y_length = max_y - min_y + 1;

        let mut result = vec![];
        for y in 0..y_length {
            let mut line = vec![];
            for x in 0..x_length {
                let color = *(self.map.get(&(x - min_x, y - min_y)).unwrap_or(&0));
                line.push(if color == 0 { " " } else { "#" });
            }
            result.push(line.join(""));
        }
        result.join("\n")
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
    let robot = Robot::new(memory).paint();
    println!("Number of individual panel painted: {}", robot.map.len());
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let memory = prepare_file(fs::read_to_string(Path::new("./data/day11.txt"))?);
    let mut robot = Robot::new(memory);
    robot.map.insert((0, 0), 1);
    robot = robot.paint();
    println!("Image is: ");
    println!("{}", robot.to_img());
    println!("Number of individual panel painted: {}", robot.map.len());
    Ok(())
}
