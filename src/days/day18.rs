use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Coordinate = (usize, usize);
type Map = HashMap<Coordinate, Tile>;
type Graph = HashMap<char, Node>;

#[derive(Debug)]
enum Tile {
    Empty,
    Wall,
    Door(char),
    Key(char),
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

fn dfs(map: &Map, start: char, nodes: &mut Graph, set_lock: bool) {
    use std::iter::FromIterator;
    let mut visited: HashMap<Coordinate, (usize, usize)> = HashMap::new();

    let start_coord = nodes.get(&start).unwrap().position;

    let mut visitable: VecDeque<(Coordinate, Vec<char>, usize)> = VecDeque::new();
    visitable.push_back((start_coord, vec![start], 0));

    let direction = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    while !visitable.is_empty() {
        let (current, gates, dist) = visitable.pop_front().unwrap();
        visited.insert(current, (dist, gates.len()));
        for movement in direction.iter() {
            let next_tile_coordinate = (
                (current.0 as isize + movement.0) as usize,
                (current.1 as isize + movement.1) as usize,
            );
            if !visited.contains_key(&next_tile_coordinate) {
                let next_tile = map.get(&next_tile_coordinate).unwrap();
                match *next_tile {
                    Empty => {
                        visitable.push_back((next_tile_coordinate, gates.clone(), dist + 1));
                    }
                    Door(door) => {
                        let mut next_gates = gates.clone();
                        next_gates.push(door.to_ascii_lowercase());
                        visitable.push_back((next_tile_coordinate, next_gates, dist + 1));
                    }
                    Key(key) => {
                        visitable.push_back((next_tile_coordinate, gates.clone(), dist + 1));
                        let start_node = nodes.get_mut(&start).unwrap();
                        start_node.distances.push((key, dist + 1));
                        if set_lock {
                            let current_node = nodes.get_mut(&key).unwrap();
                            current_node.lockers = HashSet::from_iter(gates.iter().cloned());
                        }
                    }
                    _ => {}
                }
            } else {
                let mut closer = visited.get_mut(&next_tile_coordinate).unwrap();
                if closer.0 > dist + 1 || closer.1 > gates.len() + 1 {
                    closer.0 = dist + 1;
                    closer.1 = gates.len() + 1;
                    if let Key(val) = map.get(&next_tile_coordinate).unwrap() {
                        let mut node = nodes.get_mut(&start).unwrap();
                        node.distances = node
                            .distances
                            .iter()
                            .copied()
                            .map(|x| if x.0 != *val { x } else { (x.0, dist + 1) })
                            .collect();
                        if set_lock {
                            let current_node = nodes.get_mut(&val).unwrap();
                            current_node.lockers = HashSet::from_iter(gates.iter().cloned());
                        }
                    }
                }
            }
        }
    }
}

fn find_shortest(graph: &Graph, start: char) -> usize {
    let mut min_path = None;

    let mut collected: HashMap<char, usize> = HashMap::new();
    collected.insert(start, 0);
    let node = graph.get(&start).unwrap();
    let mut rps: HashSet<String> = HashSet::new();
    let mut targets = node
        .distances
        .clone()
        .iter()
        .map(|x| {
            let mut y = HashSet::new();
            y.insert(start);
            (y, x.0, x.1)
        })
        .filter(|(_, key, _)| {
            let node = graph.get(key).unwrap();
            node.lockers.len() == 1
        })
        .collect::<Vec<(HashSet<char>, char, usize)>>();

    while !targets.is_empty() {
        let mut next_targets: Vec<_>;

        let (aquired_keys, target_key, dist) = targets.pop().unwrap();
        let mut ordered_keys = aquired_keys.iter().collect::<Vec<_>>();
        ordered_keys.sort();
        let signature = format!(
            "{}{}{}",
            ordered_keys.into_iter().collect::<String>(),
            target_key,
            dist
        );
        if !rps.insert(signature.clone()) {
            continue;
        }

        let mut newly_aquired = aquired_keys.clone();
        newly_aquired.insert(target_key);

        let node = graph.get(&target_key).unwrap();

        next_targets = node
            .distances
            .iter()
            .filter(|(next_key, _)| {
                if newly_aquired.contains(next_key) {
                    false
                } else {
                    let next_node = graph.get(next_key).unwrap();
                    next_node.lockers.is_subset(&newly_aquired)
                }
            })
            .map(|x| (newly_aquired.clone(), x.0, x.1 + dist))
            .collect();
        if next_targets.is_empty() {
            min_path = if let Some(previous_dist) = min_path {
                Some(std::cmp::min(previous_dist, dist))
            } else {
                Some(dist)
            }
        }
        targets.append(&mut next_targets);
    }
    min_path.unwrap_or(0)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (network, mut graph) = create_map(fs::read_to_string(Path::new("./data/day18.txt"))?);
    let keys = graph.keys().copied().collect::<Vec<char>>();

    for key in keys {
        dfs(&network, key, &mut graph, key == '@');
    }

    let result = find_shortest(&graph, '@');
    println!("Shortest path: {}", result);

    /*for (key, value) in graph {
        println!("{}: {:?}", key, value)
    }*/

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
