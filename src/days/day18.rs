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
    is_key: bool,
    neighbours: HashMap<char, usize>,
}

impl Node {
    fn new() -> Self {
        Node {
            lockers: HashSet::new(),
            keys: HashSet::new(),
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
        let (coordinate, mut distance, lock_state_id) = to_visit.pop_back().unwrap();

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
                    let new_lock_state_id = lock_states.len() - 1;

                    to_visit.push_front((neighbour_tile, distance, new_lock_state_id));
                }
                Door(id) => {
                    let new_lock_state =
                        format!("{}{}", lock_states[lock_state_id], id.to_ascii_lowercase());
                    lock_states.push(new_lock_state);
                    let new_lock_state_id = lock_states.len() - 1;

                    to_visit.push_front((neighbour_tile, distance, new_lock_state_id));
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
    type QueuedData<'a> = VecDeque<(Vec<&'a Node>, HashSet<char>, usize, Option<char>)>;

    let mut min = None;

    let mut dedup: HashMap<String, usize> = HashMap::new();

    let mut root_keys = HashSet::new();

    let roots: Vec<&Node> = graph
        .iter()
        .filter(|(key, _)| key.is_ascii_digit())
        .map(|(root_key, node)| {
            root_keys.insert(*root_key);
            node
        })
        .collect();

    let mut queue: QueuedData = VecDeque::new();

    queue.push_front((roots, root_keys, 0, None));
    while !queue.is_empty() {
        let (nodes, keys_aquired, dist, last_visited) = queue.pop_front().unwrap();

        if keys_aquired.len() == graph.len() {
            min = if let Some(min_value) = min {
                Some(std::cmp::min(min_value, dist))
            } else {
                Some(dist)
            }
        }

        // Dedup algorithm doesn't work on some edge case (the last test case doesn't have consistent result), thankfully, my puzzle input wasn't one of them.
        if let Some(last_char) = last_visited {
            let mut signature = keys_aquired.iter().copied().collect::<Vec<char>>();
            signature.sort();
            let signature = format!("{}{}", signature.iter().collect::<String>(), last_char);

            if let Some(dedup_min) = dedup.get(&signature) {
                if *dedup_min <= dist {
                    continue;
                }
            }

            dedup.insert(signature, dist);
        }

        for (index, node) in nodes.iter().enumerate() {
            for (node_name, node_dist) in node.neighbours.iter() {
                let next_node = graph.get(node_name).unwrap();
                if next_node.lockers.is_subset(&keys_aquired) && !keys_aquired.contains(node_name) {
                    let mut other_nodes = nodes.clone();
                    let mut new_keys_aquired = keys_aquired.clone();
                    new_keys_aquired.insert(*node_name);

                    let next_dist = dist + node_dist;
                    other_nodes[index] = next_node;
                    queue.push_back((other_nodes, new_keys_aquired, next_dist, Some(*node_name)));
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

    println!("Min dist: {}", bfs_graph_to_star(&graph));

    Ok(())
}
