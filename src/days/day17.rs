use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;

use super::intcode;

type Coordinate = (usize, usize);

#[derive(Debug, PartialEq)]
struct PathWay {
    junctions: isize,
    crossroad: Vec<Option<Coordinate>>,
}

impl PathWay {
    pub fn new() -> Self {
        PathWay {
            junctions: 0,
            crossroad: vec![None, None, None, None],
        }
    }
}

#[derive(Debug)]
struct Robot {
    facing: usize,
    position: Coordinate,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            facing: 0,
            position: (0, 0),
        }
    }
}

#[derive(Debug)]
enum Tile {
    Empty,
    Scaffold(PathWay),
    Start,
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
                _ => Start,
            };
            map.insert(coordinate, tile);
        }
    }

    println!("Sum of alignement parameters: {}", alignement);
    Ok(())
}

fn find_next_prog(path: &str) -> String {
    let splitted_path = path.split(' ').collect::<Vec<_>>();
    let mut prog = splitted_path[0..4].join(" ");
    let mut step = 4;
    let mut previous_count = 0;

    loop {
        let next_prog = vec![prog.clone(), splitted_path[step..step + 2].join(" ")].join(" ");
        let control_check = splitted_path[step - 2..step + 2].join(" ");
        let count = path.matches(&next_prog).count();
        let check_count = path.matches(&control_check).count();
        if previous_count > count || check_count != count {
            break;
        } else {
            previous_count = count;
        }

        step += 2;
        prog = next_prog;
    }

    prog
}

fn create_path(map: HashMap<Coordinate, Tile>, mut robot_on_map: Robot) -> String {
    use Tile::*;
    let mut path = String::new();
    let mut count_forward = 0;

    loop {
        let current_tile = map.get(&robot_on_map.position).unwrap();

        let left_turn = if robot_on_map.facing > 0 {
            robot_on_map.facing - 1
        } else {
            3
        };
        let right_turn = (robot_on_map.facing + 1) % 4;

        if let Scaffold(scaffold) = current_tile {
            if let Some(coordinate) = scaffold.crossroad[robot_on_map.facing] {
                count_forward += 1;
                robot_on_map.position = coordinate;
            } else if scaffold.crossroad[right_turn].is_some() {
                if count_forward == 0 {
                    path += "R";
                } else {
                    path = format!("{},{},R", path, count_forward);
                }
                count_forward = 0;
                robot_on_map.facing = right_turn;
            } else if scaffold.crossroad[left_turn].is_some() {
                if count_forward == 0 {
                    path += "L";
                } else {
                    path = format!("{},{},L", path, count_forward);
                }
                count_forward = 0;
                robot_on_map.facing = left_turn;
            } else {
                path = format!("{},{}", path, count_forward);
                break;
            }
        }
    }
    path
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    use Tile::*;
    let mut memory = intcode::prepare_memory(fs::read_to_string(Path::new("./data/day17.txt"))?);
    memory[0] = 2;
    let mut robot = intcode::Intcode::new(memory);

    let mut map: HashMap<Coordinate, Tile> = HashMap::new();

    robot = robot.run();

    let mut output = robot.get_outputs();
    output.reverse(); // Because (How could I forgot that...) inputs and outputs are store in a fifo order :D

    let ascii_map = output
        .into_iter()
        .map(|x| (x as u8) as char)
        .collect::<String>();
    let ascii_map = ascii_map.trim();

    let mut robot_on_map = Robot::new();

    for (line_no, line) in ascii_map.split('\n').enumerate() {
        for (col_no, character) in line.chars().enumerate() {
            let coordinate = (col_no, line_no);
            let tile = match character {
                '.' => Empty,
                x => {
                    let mut pathway = PathWay::new();
                    if coordinate.0 > 0 {
                        let other_coordinate = (coordinate.0 - 1, coordinate.1);
                        if let Scaffold(other_path) =
                            map.get_mut(&other_coordinate).unwrap_or(&mut Empty)
                        {
                            other_path.junctions += 1;
                            pathway.junctions += 1;

                            other_path.crossroad[1] = Some(coordinate);
                            pathway.crossroad[3] = Some(other_coordinate);
                        }
                    }
                    if coordinate.1 > 0 {
                        let other_coordinate = (coordinate.0, coordinate.1 - 1);
                        if let Scaffold(other_path) =
                            map.get_mut(&other_coordinate).unwrap_or(&mut Empty)
                        {
                            other_path.junctions += 1;
                            pathway.junctions += 1;

                            other_path.crossroad[2] = Some(coordinate);
                            pathway.crossroad[0] = Some(other_coordinate);
                        }
                    }
                    match x {
                        '^' => {
                            robot_on_map.facing = 0;
                            robot_on_map.position = coordinate;
                        }
                        '>' => {
                            robot_on_map.facing = 1;
                            robot_on_map.position = coordinate;
                        }
                        'v' => {
                            robot_on_map.facing = 2;
                            robot_on_map.position = coordinate;
                        }
                        '<' => {
                            robot_on_map.facing = 3;
                            robot_on_map.position = coordinate;
                        }
                        _ => {}
                    }
                    Scaffold(pathway)
                }
            };
            map.insert(coordinate, tile);
        }
    }

    let path = create_path(map, robot_on_map);

    let mut temp_prog = path.replace(",", " ");

    let prog_a = find_next_prog(&temp_prog);
    temp_prog = temp_prog.replace(&prog_a, "");
    let prog_b = find_next_prog(&temp_prog);
    temp_prog = temp_prog.replace(&prog_b, "");
    let prog_c = find_next_prog(&temp_prog);

    let prog_a = prog_a.trim().replace(" ", ",");
    let prog_b = prog_b.trim().replace(" ", ",");
    let prog_c = prog_c.trim().replace(" ", ",");

    let mut final_path = path.replace(&prog_a, "A");
    final_path = final_path.replace(&prog_b, "B");
    final_path = final_path.replace(&prog_c, "C");

    let main_func = format!("{}\n{}\n{}\n{}\nn\n", final_path, prog_a, prog_b, prog_c)
        .chars()
        .map(|x| (x as u8) as i64)
        .collect::<Vec<i64>>();

    for i in main_func {
        robot = robot.add_input(i);
    }

    robot = robot.run();

    let mut output = robot.get_outputs();
    output.reverse();
    let result = output.pop().unwrap();

    println!("Result: {}", result);
    Ok(())
}
