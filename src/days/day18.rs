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
    Start(char),
}

use Tile::*;

#[derive(Debug)]
struct Node {
    lockers: HashSet<char>,
    keys: HashSet<char>,
    dist: usize,
    is_key: bool,
    neighbours: HashMap<char, usize>,
}

impl Node {
    fn new() -> Self {
        Node {
            lockers: HashSet::new(),
            keys: HashSet::new(),
            dist: 0,
            is_key: false,
            neighbours: HashMap::new(),
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            lockers: self.lockers.clone(),
            neighbours: self.neighbours.clone(),
            dist: self.dist,
            is_key: self.is_key,
            keys: self.keys.clone(),
        }
    }
}

fn create_map(input: String) -> Map {
    let mut network: HashMap<(usize, usize), Tile> = HashMap::new();
    let mut start_count = 0;
    for (line_no, line) in input.split('\n').enumerate() {
        let line = line.trim();
        for (col_no, character) in line.chars().enumerate() {
            let coordinate = (col_no, line_no);
            let tile = match character {
                '.' => Empty,
                '#' => Wall,
                '@' => {
                    let id = start_count.to_string().chars().next().unwrap();
                    start_count += 1;
                    Start(id)
                }
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

fn bfs_from_node(map: &Map, start: Coordinate, is_root: bool, graph: &mut Graph) {
    use std::iter::FromIterator;
    let mut to_visit: VecDeque<(Coordinate, usize, usize)> = VecDeque::new();
    to_visit.push_back((start, 0, 0));

    let mut already_discovered: HashSet<Coordinate> = HashSet::new();
    already_discovered.insert(start);

    let mut lock_states = vec![String::new()];

    let movements = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let node_id = match map.get(&start).unwrap() {
        Start(id) | Key(id) => *id,
        _ => unreachable!(),
    };

    let mut node = Node::new();
    if let Some(previous_node) = graph.get(&node_id) {
        node.lockers = previous_node.lockers.clone();
    }

    while !to_visit.is_empty() {
        let (coordinate, mut distance, mut lock_state_id) = to_visit.pop_back().unwrap();

        distance += 1;

        for movement in movements.iter() {
            let neighbour_tile = (
                (coordinate.0 as isize + movement.0) as usize,
                (coordinate.1 as isize + movement.1) as usize,
            );

            if !already_discovered.insert(neighbour_tile) {
                continue;
            }

            match map.get(&neighbour_tile).unwrap() {
                Start(id) | Key(id) => {
                    node.neighbours.insert(*id, distance);

                    if is_root {
                        let mut remote_node = graph.entry(*id).or_insert_with(Node::new);
                        remote_node.lockers =
                            HashSet::from_iter(lock_states[lock_state_id].chars());
                    }

                    let new_lock_state =
                        format!("{}{}", lock_states[lock_state_id], id.to_ascii_lowercase());
                    lock_states.push(new_lock_state);
                    lock_state_id = lock_states.len() - 1;

                    to_visit.push_front((neighbour_tile, distance, lock_state_id));
                }
                Door(id) => {
                    let new_lock_state =
                        format!("{}{}", lock_states[lock_state_id], id.to_ascii_lowercase());
                    lock_states.push(new_lock_state);
                    lock_state_id = lock_states.len() - 1;

                    to_visit.push_front((neighbour_tile, distance, lock_state_id));
                }
                Empty => {
                    to_visit.push_front((neighbour_tile, distance, lock_state_id));
                }
                Wall => {}
            }
        }
    }

    graph.insert(node_id, node);
}

fn bfs_graph_to_star(graph: &Graph) -> usize {
    let mut min = None;

    let mut dedup: HashMap<String, usize> = HashMap::new();

    let mut queue: VecDeque<(Node, char)> = graph
        .iter()
        .filter(|(key, _)| key.is_ascii_digit())
        .map(|(key, start)| (start.clone(), *key))
        .collect();
    while !queue.is_empty() {
        let (node, last_visited) = queue.pop_front().unwrap();
        if node.keys.len() == graph.len() {
            min = if let Some(min_value) = min {
                Some(std::cmp::min(min_value, node.dist))
            } else {
                Some(node.dist)
            }
        }

        let mut signature = node.keys.iter().copied().collect::<Vec<char>>();

        signature.sort();
        let signature = format!("{}{}", signature.iter().collect::<String>(), last_visited);

        if let Some(dedup_min) = dedup.get(&signature) {
            if *dedup_min <= node.dist {
                continue;
            }
        }

        dedup.insert(signature, node.dist);

        for (node_name, dist) in node.neighbours.iter() {
            let next_node = graph.get(node_name).unwrap();
            if next_node.lockers.is_subset(&node.keys) && !node.keys.contains(node_name) {
                let mut new_node = next_node.clone();
                new_node.dist = node.dist + dist;
                new_node.keys = node.keys.clone();
                new_node.keys.insert(*node_name);
                queue.push_back((new_node, *node_name));
            }
        }
    }

    min.unwrap_or(0)
}

fn bfs_graph_to_star_2(graph: &Graph) -> usize {
    let mut min = None;

    let mut dedup: HashMap<String, usize> = HashMap::new();

    let roots: Vec<Node> = graph
        .iter()
        .filter(|(key, _)| key.is_ascii_digit())
        .map(|(_, start)| start.clone())
        .collect();

    let mut queue: VecDeque<(Vec<Node>, Option<char>)> = VecDeque::new();
    queue.push_front((roots, None));
    while !queue.is_empty() {
        let (nodes, last_visited) = queue.pop_front().unwrap();

        let first_node = &nodes[0];

        if first_node.keys.len() == graph.len() {
            min = if let Some(min_value) = min {
                Some(std::cmp::min(min_value, first_node.dist))
            } else {
                Some(first_node.dist)
            }
        }

        if let Some(last_char) = last_visited {
            let mut signature = first_node.keys.iter().copied().collect::<Vec<char>>();
            signature.sort();
            let signature = format!("{}{}", signature.iter().collect::<String>(), last_char);

            if let Some(dedup_min) = dedup.get(&signature) {
                if *dedup_min <= first_node.dist {
                    continue;
                }
            }

            dedup.insert(signature, first_node.dist);
        }

        for (index, node) in nodes.iter().enumerate() {
            for (node_name, dist) in node.neighbours.iter() {
                let next_node = graph.get(node_name).unwrap();
                if next_node.lockers.is_subset(&node.keys) && !node.keys.contains(node_name) {
                    let mut other_nodes = nodes.clone();
                    for (index_other, other_node) in other_nodes.iter_mut().enumerate() {
                        if index_other != index {
                            other_node.dist = node.dist + dist;
                            other_node.keys.insert(*node_name);
                        }
                    }

                    let mut new_node = next_node.clone();

                    new_node.dist = node.dist + dist;
                    new_node.keys = node.keys.clone();
                    new_node.keys.insert(*node_name);

                    other_nodes[index] = new_node;
                    queue.push_back((other_nodes, Some(*node_name)));
                }
            }
        }
    }

    min.unwrap_or(0)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let map = create_map(fs::read_to_string(Path::new("./data/day18_a.txt"))?);
    let mut graph = Graph::new();

    for (coord, tile) in map.iter().filter(|(_, tile)| match tile {
        Key(_) | Start(_) => true,
        _ => false,
    }) {
        let is_root = match tile {
            Start(_) => true,
            Key(_) => false,
            _ => unreachable!(),
        };
        bfs_from_node(&map, *coord, is_root, &mut graph);
    }

    println!("Min dist: {}", bfs_graph_to_star(&graph));

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let map = create_map(fs::read_to_string(Path::new("./data/day18_b.txt"))?);
    let mut graph = Graph::new();

    for (coord, tile) in map.iter().filter(|(_, tile)| match tile {
        Key(_) | Start(_) => true,
        _ => false,
    }) {
        let is_root = match tile {
            Start(_) => true,
            Key(_) => false,
            _ => unreachable!(),
        };
        bfs_from_node(&map, *coord, is_root, &mut graph);
    }

    println!("Min dist: {}", bfs_graph_to_star_2(&graph));

    Ok(())
}
// 2478 too high