use std::error::Error;
use std::fs;
use std::path::Path;

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Recipe {
    qut_produced: u64,
    composition: HashMap<String, u64>,
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

        let mut composition: HashMap<String, u64> = HashMap::new();

        for ins in going_in {
            let qut = ins[1].parse::<u64>().unwrap();
            let name = ins[2].to_string();

            composition.insert(name, qut);
        }

        values.insert(
            out_name,
            Recipe {
                qut_produced: going_out[1].parse::<u64>().unwrap(),
                composition,
            },
        );
    }

    values
}

fn run_machine(recipes: &HashMap<String, Recipe>, fuel_to_produce: u64) -> u64 {
    let mut status = recipes.get("FUEL").unwrap().composition.clone();
    let mut left_overs: HashMap<String, u64> = HashMap::new();

    let mut ore_count = 0;

    for (_, qut) in status.iter_mut() {
        *qut *= fuel_to_produce;
    }

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

            let ratio = (truly_needed as f64 / recipe.qut_produced as f64).ceil() as u64;

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
    ore_count
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let recipes = get_recipes(fs::read_to_string(Path::new("./data/day14.txt"))?);
    println!(
        "Minimum quantity of ORE required: {}",
        run_machine(&recipes, 1)
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let available_ore: u64 = 1_000_000_000_000;
    let recipes = get_recipes(fs::read_to_string(Path::new("./data/day14.txt"))?);

    let fuel_for_ore = run_machine(&recipes, 1);

    let mut attempt = 0;
    let mut left_overs_ore = available_ore;

    loop {
        let fuel_to_add = if left_overs_ore / fuel_for_ore == 0 {
            1
        } else {
            left_overs_ore / fuel_for_ore
        };
        attempt += fuel_to_add;
        let ore_consumed = run_machine(&recipes, attempt);

        if ore_consumed < available_ore {
            left_overs_ore = available_ore - ore_consumed;
        } else {
            attempt -= fuel_to_add;
            break;
        }
    }

    println!("Maximum fuel: {}", attempt);
    Ok(())
}
