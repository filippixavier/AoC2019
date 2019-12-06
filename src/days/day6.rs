use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;

pub fn prepare_input(input: String) -> HashMap<String, Vec<String>> {
    let nodes = input
        .trim()
        .split('\n')
        .map(|elem| {
            elem.trim()
                .split(')')
                .map(|part| part.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let mut tree: HashMap<String, Vec<String>> = HashMap::new();

    for node in nodes {
        let childs = tree.entry(node[0].clone()).or_insert_with(|| vec![]);
        childs.push(node[1].clone());
    }

    tree
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let tree = prepare_input(fs::read_to_string(Path::new("./data/day6.txt"))?);
    let mut count = 0;

    bfs_first_star(&tree, vec!["COM".to_string()], 0, &mut count);

    println!("Number of orbits: {}", count);

    Ok(())
}

fn bfs_first_star(
    tree: &HashMap<String, Vec<String>>,
    same_depth_nodes: Vec<String>,
    depth: usize,
    count: &mut usize,
) {
    let mut next_depth = vec![];

    *count += same_depth_nodes.len() * depth;

    for node in same_depth_nodes {
        if let Some(childs) = tree.get(&node) {
            let mut cloned_child = childs.clone();
            next_depth.append(&mut cloned_child);
        }
    }

    if !next_depth.is_empty() {
        bfs_first_star(tree, next_depth, depth + 1, count);
    }
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let tree = prepare_input(fs::read_to_string(Path::new("./data/day6.txt"))?);

    // I could try and implement a tree with backreference and use an hashset but it's kind of a pain soo...
    let (mut com_to_san, _) = route_to(&tree, &"SAN".to_string(), vec!["COM".to_string()]);
    let (mut com_to_me, _) = route_to(&tree, &"YOU".to_string(), vec!["COM".to_string()]);

    let mut san_to_me: Vec<String> = vec![];

    com_to_san.reverse();
    com_to_me.reverse();

    loop {
        let left = com_to_san.last().unwrap();
        let right = com_to_me.last().unwrap();

        if left == right {
            com_to_me.pop();
            com_to_san.pop();
        } else {
            san_to_me.append(&mut com_to_san);
            san_to_me.append(&mut com_to_me);
            break;
        }
    }

    println!("I need {} jumps to join Santa", san_to_me.len() - 2);

    Ok(())
}

fn route_to(
    tree: &HashMap<String, Vec<String>>,
    target: &str,
    mut route: Vec<String>,
) -> (Vec<String>, bool) {
    if let Some(node) = route.last() {
        if node == target {
            return (route, true);
        }

        if let Some(childs) = tree.get(node) {
            for child in childs {
                route.push(child.to_string());
                let (temp_route, result) = route_to(tree, target, route);
                route = temp_route;
                if result {
                    return (route, result);
                }
                route.pop();
            }
        }
    }

    (route, false)
}
