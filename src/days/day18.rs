use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Coordinate = (usize, usize);
type Map = HashMap<Coordinate, Tile>;
type Graph = HashMap<char, Node>;

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Door(char),
    Key(char),
    Start,
}

use Tile::*;

#[derive(Debug)]
struct Node {
    lockers: HashSet<char>,
    distances: Vec<(char, usize)>,
    position: Coordinate,
}

impl Node {
    fn new(position: Coordinate) -> Self {
        Node {
            lockers: HashSet::new(),
            distances: vec![],
            position,
        }
    }
}

fn create_map(input: String) -> (Map, Graph) {
    let mut network: HashMap<(usize, usize), Tile> = HashMap::new();
    let mut graph: Graph = HashMap::new();

    for (line_no, line) in input.split('\n').enumerate() {
        let line = line.trim();
        for (col_no, character) in line.chars().enumerate() {
            let coordinate = (col_no, line_no);
            let tile = match character {
                '.' => Empty,
                '#' => Wall,
                '@' => Start,
                x => {
                    let node = Node::new((col_no, line_no));
                    if x.is_ascii_uppercase() {
                        Door(x)
                    } else {
                        graph.insert(x, node);
                        Key(x)
                    }
                }
            };

            network.insert(coordinate, tile);
        }
    }

    (network, graph)
}

fn find_bfs_star_1(map: &Map, num_of_keys: usize) -> usize {
    let mut visitable = map
        .iter()
        .filter(|(_, val)| **val == Start)
        .map(|(key, _)| (*key, 0, HashSet::new()))
        .collect::<VecDeque<(Coordinate, usize, HashSet<char>)>>();

    let mut visited: HashSet<String> = HashSet::new();

    let direction = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    loop {
        if visitable.is_empty() {
            return 666;
        }
        let (current_pos, current_dist, keys_list) = visitable.pop_front().unwrap();
        let mut keys_list_sorted = keys_list.iter().copied().collect::<Vec<char>>();
        keys_list_sorted.sort();
        let signature = format!(
            "{}_{}_{}",
            current_pos.0,
            current_pos.1,
            keys_list_sorted.iter().collect::<String>()
        );

        if visitable.is_empty() {
            println!(
                "From: {:?}, {:?}, {}, {:?}",
                map.get(&current_pos).unwrap(),
                current_pos,
                current_dist,
                keys_list
            );
        }

        if !visited.insert(signature.clone()) {
            continue;
        }

        for movement in direction.iter() {
            let next_tile_coordinate = (
                (current_pos.0 as isize + movement.0) as usize,
                (current_pos.1 as isize + movement.1) as usize,
            );

            let tile = map.get(&next_tile_coordinate).unwrap();
            match tile {
                Key(key) => {
                    let mut keys_list = keys_list.clone();
                    keys_list.insert(*key);
                    if keys_list.len() == num_of_keys {
                        return current_dist + 1;
                    }
                    visitable.push_back((next_tile_coordinate, current_dist + 1, keys_list));
                }
                Door(lock) => {
                    let required_key = lock.to_ascii_lowercase();
                    if keys_list.contains(&required_key) {
                        visitable.push_back((
                            next_tile_coordinate,
                            current_dist + 1,
                            keys_list.clone(),
                        ));
                    }
                }
                Wall => {}
                _ => {
                    visitable.push_back((
                        next_tile_coordinate,
                        current_dist + 1,
                        keys_list.clone(),
                    ));
                }
            }
        }
    }
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (network, graph) = create_map(fs::read_to_string(Path::new("./data/day18_a.txt"))?);
    let result = find_bfs_star_1(&network, graph.len());
    println!("Shortest path: {}", result);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
