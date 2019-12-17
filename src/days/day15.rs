use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;

use super::intcode;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty(usize),
    Wall,
    Oxygen(usize),
    Unknown,
    Start,
}

type Coordinate = (i32, i32);
type Map = HashMap<Coordinate, Tile>;

fn explore_map(mut repair_drone: intcode::Intcode) -> (Map, Tile, Coordinate) {
    use self::Tile::*;

    let mut position = (0, 0);
    let mut area: Map = HashMap::new();

    let mut dist = 0;
    let steps: Vec<i64> = vec![3, 1, 4, 2];
    let steps_back: Vec<i64> = vec![-1, 2, 1, 4, 3];
    let mut backward: Vec<i64> = vec![];
    let mut previous_positions: Vec<Coordinate> = vec![];

    let mut oxygen_tile = Unknown;
    let mut oxygen_position = None;

    area.insert(position, Start);

    loop {
        let mut available_step = vec![];
        for next_step in steps.iter() {
            let next_move = match next_step {
                1 => (0, -1),
                2 => (0, 1),
                3 => (-1, 0),
                4 => (1, 0),
                _ => unreachable!(),
            };
            let next_pos = (position.0 + next_move.0, position.1 + next_move.1);
            let next_tile = *(area.entry(next_pos).or_insert(Unknown));
            match next_tile {
                Unknown => available_step.push((*next_step, next_pos)),
                Oxygen(recorded_dist) | Empty(recorded_dist) => {
                    if recorded_dist > dist + 1 {
                        available_step.push((*next_step, next_pos));
                    }
                }
                _ => {}
            }
        }

        if available_step.is_empty() {
            if backward.is_empty() {
                break;
            } else {
                let previous_step = backward.pop().unwrap();
                position = previous_positions.pop().unwrap();
                repair_drone = repair_drone.add_input(previous_step).run();
                repair_drone.get_outputs();
                dist -= 1;
                continue;
            }
        }

        for _ in 0..available_step.len() {
            let (next_step, next_pos) = available_step.pop().unwrap();
            repair_drone = repair_drone.add_input(next_step).run();

            let has_moved = match repair_drone.get_outputs()[0] {
                0 => {
                    area.insert(next_pos, Wall);
                    false
                }
                1 => {
                    area.insert(next_pos, Empty(dist + 1));
                    true
                }
                2 => {
                    area.insert(next_pos, Oxygen(dist + 1));
                    oxygen_tile = Oxygen(dist + 1);
                    oxygen_position = Some(next_pos);
                    true
                }
                _ => unreachable!(),
            };

            if has_moved {
                previous_positions.push(position);
                position = next_pos;
                dist += 1;
                backward.push(steps_back[next_step as usize]);
                break;
            }
        }
    }

    (area, oxygen_tile, oxygen_position.unwrap())
}

fn draw_map(area: &Map) {
    use Tile::*;
    let mut min_x: Option<i32> = None;
    let mut min_y: Option<i32> = None;
    let mut max_x: Option<i32> = None;
    let mut max_y: Option<i32> = None;
    for (x, y) in area.keys() {
        min_x = if let Some(min_x) = min_x {
            Some(std::cmp::min(*x, min_x))
        } else {
            Some(*x)
        };

        min_y = if let Some(min_y) = min_y {
            Some(std::cmp::min(*y, min_y))
        } else {
            Some(*y)
        };

        max_x = if let Some(max_x) = max_x {
            Some(std::cmp::max(*x, max_x))
        } else {
            Some(*x)
        };

        max_y = if let Some(max_y) = max_y {
            Some(std::cmp::max(*y, max_y))
        } else {
            Some(*y)
        };
    }

    for y in min_y.unwrap()..=max_y.unwrap() {
        for x in min_x.unwrap()..=max_x.unwrap() {
            let tile = area.get(&(x, y)).unwrap_or(&Unknown);
            print!(
                "{}",
                match tile {
                    Empty(_) => '.',
                    Wall => '#',
                    Oxygen(_) => 'O',
                    Unknown => '?',
                    Start => 'X',
                }
            );
        }
        println!();
    }
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (area, oxygen_tile, _) = explore_map(intcode::Intcode::new_with_path(fs::read_to_string(
        Path::new("./data/day15.txt"),
    )?));
    draw_map(&area);

    if let Tile::Oxygen(dist) = oxygen_tile {
        println!("Oxygen found at dist: {}", dist);
    }

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    use Tile::*;
    let (mut area, _, position) = explore_map(intcode::Intcode::new_with_path(fs::read_to_string(
        Path::new("./data/day15.txt"),
    )?));

    area.insert(position, Oxygen(0));

    let mut to_explore = vec![position];
    let moves = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut max_dist = 0;

    while !to_explore.is_empty() {
        let oxy_position = to_explore.pop().unwrap();

        let dist = if let Oxygen(dist_to_origin) = area.get(&oxy_position).unwrap() {
            *dist_to_origin
        } else {
            0
        };

        if dist > max_dist {
            max_dist = dist;
        }

        for movement in moves.iter() {
            let neighbor = (oxy_position.0 + movement.0, oxy_position.1 + movement.1);
            if let Some(empty_cell) = area.get_mut(&neighbor) {
                *empty_cell = match empty_cell {
                    Oxygen(previous_dist) => {
                        if *previous_dist < dist + 1 {
                            Oxygen(*previous_dist)
                        } else {
                            Oxygen(dist + 1)
                        }
                    }
                    Empty(_) => {
                        to_explore.push(neighbor);
                        Oxygen(dist + 1)
                    }
                    Unknown => Unknown,
                    Start => Oxygen(dist + 1),
                    Wall => Wall,
                }
            }
        }
    }

    println!("It takes {} mins to fully fill the area", max_dist);
    Ok(())
}
