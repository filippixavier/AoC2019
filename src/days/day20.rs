use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Coordinate = (usize, usize);

type CharMaze = Vec<Vec<char>>;

type Maze = HashMap<Coordinate, Tile>;
type Gates = HashMap<String, Coordinate>;
type Warps = HashMap<Coordinate, (Coordinate, bool)>;

#[derive(Debug)]
enum Tile {
    Void,
    Empty,
    Wall,
}

fn find_portal(
    maze: &[Vec<char>],
    start: Coordinate,
    partial_gate_list: &mut HashSet<Coordinate>,
    gate_list: &mut Gates,
    warps: &mut Warps,
    is_inner_ring: bool,
) {
    let start_tile = maze[start.0][start.1];

    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut start_pos = None;

    let mut exit = start;
    let mut door = String::new();

    for modifier in directions {
        let neighbor = (start.0 as isize + modifier.0, start.1 as isize + modifier.1);

        if neighbor.0 >= 0
            && neighbor.0 < maze.len() as isize
            && neighbor.1 >= 0
            && neighbor.1 < maze[0].len() as isize
        {
            let neighbor = (neighbor.0 as usize, neighbor.1 as usize);
            let tile = maze[neighbor.0][neighbor.1];

            if tile.is_alphabetic() {
                exit = neighbor;
                partial_gate_list.insert(neighbor);
                door = if neighbor < start {
                    format!("{}{}", tile, start_tile)
                } else {
                    format!("{}{}", start_tile, tile)
                }
            } else if tile == '.' {
                start_pos = Some(neighbor);
            }
        }
    }

    let is_lateral = start.0 == exit.0;

    let gate_coordinate = if let Some(gate) = start_pos {
        gate
    } else if is_lateral {
        (exit.0, exit.1 + 1)
    } else {
        (exit.0 + 1, exit.1)
    };

    if let Some(twin_gate) = gate_list.get(&door) {
        warps.insert(gate_coordinate, (*twin_gate, is_inner_ring));
        warps.insert(*twin_gate, (gate_coordinate, !is_inner_ring));
    } else {
        gate_list.insert(door, gate_coordinate);
    }
}

fn prepare_file(input: String) -> (Maze, Warps, Coordinate, Coordinate) {
    use self::Tile::*;

    let mut partial_gate_list: HashSet<Coordinate> = HashSet::new();
    let mut unlinked_gates = Gates::new();
    let mut linked_gates = Warps::new();

    let mut maze = Maze::new();

    let lines = input.lines().collect::<Vec<_>>();
    let char_maze = lines
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<CharMaze>();

    for (line_no, line) in char_maze.iter().enumerate() {
        for (col_no, tile) in line.iter().enumerate() {
            let coordinate = (line_no, col_no);
            match tile {
                ' ' => {
                    maze.insert(coordinate, Void);
                }
                '#' => {
                    maze.insert(coordinate, Wall);
                }
                '.' => {
                    maze.insert(coordinate, Empty);
                }
                letter => {
                    if !letter.is_alphabetic() {
                        continue;
                    }

                    let is_inner_ring = line_no > 2
                        && line_no < char_maze.len() - 2
                        && col_no > 2
                        && col_no < char_maze[0].len() - 2;

                    if partial_gate_list.insert(coordinate) {
                        find_portal(
                            &char_maze,
                            coordinate,
                            &mut partial_gate_list,
                            &mut unlinked_gates,
                            &mut linked_gates,
                            is_inner_ring,
                        );
                    }

                    maze.insert(coordinate, Void);
                }
            }
        }
    }

    (
        maze,
        linked_gates,
        *unlinked_gates.get("AA").unwrap(),
        *unlinked_gates.get("ZZ").unwrap(),
    )
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    use self::Tile::*;

    let (maze, warps, start_point, end_point) =
        prepare_file(fs::read_to_string(Path::new("./data/day20.txt"))?);

    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut already_explored: HashSet<Coordinate> = HashSet::new();
    let mut to_explore = VecDeque::new();

    to_explore.push_front((start_point, 0));

    while !to_explore.is_empty() {
        let (current_coordinate, dist) = to_explore.pop_front().unwrap();

        if !already_explored.insert(current_coordinate) {
            continue;
        }

        if current_coordinate == end_point {
            println!("Total dist from AA to ZZ: {}", dist);
            break;
        }

        for direction in directions.iter() {
            let neighbor = (
                (current_coordinate.0 as isize + direction.0) as usize,
                (current_coordinate.1 as isize + direction.1) as usize,
            );

            match maze.get(&neighbor).unwrap() {
                Empty => {
                    to_explore.push_back((neighbor, dist + 1));
                }
                Void => {
                    if warps.contains_key(&current_coordinate) {
                        let (coordinate, _) = *warps.get(&current_coordinate).unwrap();
                        to_explore.push_back((coordinate, dist + 1));
                    }
                }
                Wall => {}
            }
        }
    }
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    use self::Tile::*;

    let (maze, warps, start_point, end_point) =
        prepare_file(fs::read_to_string(Path::new("./data/day20.txt"))?);

    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut already_explored: HashSet<(Coordinate, usize)> = HashSet::new();
    let mut to_explore = VecDeque::new();

    to_explore.push_front((start_point, 0, 0));

    while !to_explore.is_empty() {
        let (current_coordinate, dist, depth) = to_explore.pop_front().unwrap();

        if !already_explored.insert((current_coordinate, depth)) {
            continue;
        }

        if current_coordinate == end_point && depth == 0 {
            println!("Total dist from AA to ZZ: {}", dist);
            break;
        }

        for direction in directions.iter() {
            let neighbor = (
                (current_coordinate.0 as isize + direction.0) as usize,
                (current_coordinate.1 as isize + direction.1) as usize,
            );

            match maze.get(&neighbor).unwrap() {
                Empty => {
                    to_explore.push_back((neighbor, dist + 1, depth));
                }
                Void => {
                    if warps.contains_key(&current_coordinate) {
                        let (coordinate, is_inner_ring) = *warps.get(&current_coordinate).unwrap();

                        if depth == 0 && !is_inner_ring {
                            continue;
                        }

                        let new_depth = if is_inner_ring { depth + 1 } else { depth - 1 };

                        to_explore.push_back((coordinate, dist + 1, new_depth));
                    }
                }
                Wall => {}
            }
        }
    }
    Ok(())
}
