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
    keys: HashSet<char>,
    neighbours: HashMap<char, usize>,
    targets: HashMap<char, usize>,
}

impl Node {
    fn new() -> Self {
        Node {
            lockers: HashSet::new(),
            keys: HashSet::new(),
            neighbours: HashMap::new(),
            targets: HashMap::new(),
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            lockers: self.lockers.clone(),
            neighbours: self.neighbours.clone(),
            keys: self.keys.clone(),
            targets: HashMap::new(),
        }
    }
}

fn create_map(input: String) -> Map {
    let mut network: HashMap<(usize, usize), Tile> = HashMap::new();

    for (line_no, line) in input.split('\n').enumerate() {
        let line = line.trim();
        for (col_no, character) in line.chars().enumerate() {
            let coordinate = (col_no, line_no);
            let tile = match character {
                '.' => Empty,
                '#' => Wall,
                '@' => Start,
                x => {
                    if x.is_ascii_uppercase() {
                        Door(x)
                    } else {
                        Key(x)
                    }
                }
            };

            network.insert(coordinate, tile);
        }
    }

    network
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
            panic!("Cannot move further yet haven't found all the keys");
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

fn bfs_map_to_graph(map: &Map) -> Graph {
    use std::iter::FromIterator;
    let mut graph = Graph::new();
    let mut to_visit = map
        .iter()
        .filter(|(_, val)| **val == Start)
        .enumerate()
        .map(|(index, (key, _))| {
            (
                *key,
                0,
                vec![],
                format!("{}", index).chars().next().unwrap(),
            )
        })
        .collect::<VecDeque<(Coordinate, usize, Vec<char>, char)>>();

    let direction = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut already_visited: HashSet<Coordinate> = HashSet::new();
    while !to_visit.is_empty() {
        let (current_pos, current_dist, current_locks, origin_node) = to_visit.pop_front().unwrap();
        let next_dist = current_dist + 1;
        for movement in direction.iter() {
            let next_pos = (
                (current_pos.0 as isize + movement.0) as usize,
                (current_pos.1 as isize + movement.1) as usize,
            );

            if !already_visited.insert(next_pos) {
                continue;
            }

            match map.get(&next_pos).unwrap() {
                Wall => {}
                Door(lock) => {
                    let mut new_locks = current_locks.clone();
                    new_locks.push(lock.to_ascii_lowercase());
                    to_visit.push_back((next_pos, next_dist, new_locks, origin_node))
                }
                Key(node_name) => {
                    let previous_node = graph.entry(origin_node).or_insert(Node::new());
                    previous_node.neighbours.insert(*node_name, next_dist);

                    let mut node = Node::new();
                    node.lockers = HashSet::from_iter(current_locks.iter().copied());
                    node.neighbours.insert(origin_node, next_dist);
                    graph.insert(*node_name, node);

                    to_visit.push_back((next_pos, 0, vec![], *node_name));
                }
                _ => {
                    to_visit.push_back((next_pos, next_dist, current_locks.clone(), origin_node));
                }
            }
        }
    }

    graph
}

fn bfs_graph_to_star(graph: &Graph) {
    let mut queue: VecDeque<Node> = graph
        .iter()
        .filter(|(key, _)| key.is_ascii_digit())
        .map(|(_, start)| {let start = start.clone();
        start.targets = start.neighbours.clone()
    start})
        .collect();
    
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
    }
}

fn find_bfs_star_2(map: &Map, num_of_keys: usize) -> usize {
    let start_points = map
        .iter()
        .filter(|(_, val)| **val == Start)
        .map(|(key, _)| *key)
        .collect::<Vec<Coordinate>>();

    let mut visited: HashSet<String> = HashSet::new();

    let mut visitable: VecDeque<(Vec<Coordinate>, usize, HashSet<char>)> = VecDeque::new();
    visitable.push_front((start_points, 0, HashSet::new()));

    let direction = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    loop {
        if visitable.is_empty() {
            panic!("Cannot move further yet haven't found all the keys");
        }
        let (all_pos, current_dist, keys_list) = visitable.pop_front().unwrap();

        let mut keys_list_sorted = keys_list.iter().copied().collect::<Vec<char>>();
        keys_list_sorted.sort();

        let signed_pos = all_pos.iter().fold(String::new(), |acc, pos| {
            format!("{}_{}_{}", acc, pos.0, pos.1)
        });

        let signature = format!(
            "{}_{}",
            signed_pos,
            keys_list_sorted.iter().collect::<String>()
        );
        if !visited.insert(signature.clone()) {
            continue;
        }

        for (index, pos) in all_pos.iter().enumerate() {
            for movement in direction.iter() {
                let next_tile_coordinate = (
                    (pos.0 as isize + movement.0) as usize,
                    (pos.1 as isize + movement.1) as usize,
                );

                let mut new_pos = all_pos.clone();
                new_pos[index] = next_tile_coordinate;

                let tile = map.get(&next_tile_coordinate).unwrap();
                match tile {
                    Key(key) => {
                        let mut keys_list = keys_list.clone();
                        keys_list.insert(*key);
                        if keys_list.len() == num_of_keys {
                            return current_dist + 1;
                        }
                        visitable.push_back((new_pos, current_dist + 1, keys_list));
                    }
                    Door(lock) => {
                        let required_key = lock.to_ascii_lowercase();
                        if keys_list.contains(&required_key) {
                            visitable.push_back((new_pos, current_dist + 1, keys_list.clone()));
                        }
                    }
                    Wall => {}
                    _ => {
                        visitable.push_back((new_pos, current_dist + 1, keys_list.clone()));
                    }
                }
            }
        }
    }
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let map = create_map(fs::read_to_string(Path::new("./data/day18_a.txt"))?);
    /*let result = find_bfs_star_1(&map, graph.len());
    println!("Shortest path: {}", result);*/
    let graph = bfs_map_to_graph(&map);

    for (key, val) in graph.iter() {
        println!("{}: {:?}", key, val);
    }

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    // let (map, graph) = create_map(fs::read_to_string(Path::new("./data/day18_b.txt"))?);
    // let result = find_bfs_star_2(&map, graph.len());
    // println!("Shortest path: {}", result);

    Ok(())
}
