use std::error::Error;
use std::fs;
use std::path::Path;

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Recipe {
    qut_produced: usize,
    composition: HashMap<String, usize>,
}

fn get_recipes(input: String) -> HashMap<String, Recipe> {
    let reg = Regex::new(r"(\d+) (\w+)").unwrap();
    let mut values: HashMap<String, Recipe> = HashMap::new();

    let input = input.trim().split('\n');

    for line in input {
        let recipe = line.split("=>").collect::<Vec<_>>();
        let going_in = reg.captures_iter(recipe[0]);
        let going_out = reg.captures(recipe[1]).unwrap();
        let out_name: String = going_out[2].to_string();

        let mut composition: HashMap<String, usize> = HashMap::new();

        for ins in going_in {
            let qut = ins[1].parse::<usize>().unwrap();
            let name = ins[2].to_string();

            composition.insert(name, qut);
        }

        values.insert(
            out_name,
            Recipe {
                qut_produced: going_out[1].parse::<usize>().unwrap(),
                composition,
            },
        );
    }

    values
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let recipes = get_recipes(fs::read_to_string(Path::new("./data/day14.txt"))?);
    let mut status = recipes.get("FUEL").unwrap().composition.clone();
    let mut left_overs: HashMap<String, usize> = HashMap::new();

    let mut ore_count = 0;

    while !status.is_empty() {
        let mut new_status = HashMap::new();

        for (comp_name, qut_required) in status.iter() {
            let recipe = recipes.get(comp_name).unwrap();
            let compo_made = left_overs.entry(comp_name.clone()).or_insert(0);

            let truly_needed = if *compo_made >= *qut_required {
                *compo_made -= *qut_required;
                0
            } else {
                let value = *qut_required - *compo_made;
                *compo_made = 0;
                value
            };

            let ratio = (truly_needed as f32 / recipe.qut_produced as f32).ceil() as usize;

            *compo_made += ratio * recipe.qut_produced - truly_needed;

            for (ingredient, qut) in recipe.composition.iter() {
                if ingredient == "ORE" {
                    ore_count += qut * ratio;
                } else {
                    let current_count = new_status.entry(ingredient.clone()).or_insert(0);
                    *current_count += qut * ratio;
                }
            }
        }

        status = new_status;
    }

    println!("Minimum quantity of ORE required: {}", ore_count);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
