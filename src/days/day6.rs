use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;

pub fn prepare_input(input: String) -> Vec<Vec<String>> {
    input
        .trim()
        .split('\n')
        .map(|elem| {
            elem
                .trim()
                .split(')')
                .map(|part| part.to_string())
                .collect::<Vec<String>>()
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let nodes = prepare_input(fs::read_to_string(Path::new("./data/day6.txt"))?);

    let mut tree: HashMap<String, Vec<String>> = HashMap::new();

    for node in nodes {
        let childs = tree.entry(node[0].clone()).or_insert(vec![]);
        childs.push(node[1].clone());
    }


    let mut count = 0;

    bfs_first_star(&tree, vec!("COM".to_string()), 0, &mut count);

    println!("Number of orbits: {}", count);

    Ok(())
}

pub fn bfs_first_star(tree: &HashMap<String, Vec<String>>, same_depth_nodes: Vec<String>, depth: usize, count: &mut usize) {
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
    Ok(())
}