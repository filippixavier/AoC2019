use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Moon {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Moon {
            x,
            y,
            z,
            vx: 0,
            vy: 0,
            vz: 0,
        }
    }

    fn get_gravity_variation(&self, other: &Self) -> (i64, i64, i64) {
        let mut result = (0, 0, 0);
        result.0 = if self.x < other.x {
            1
        } else if self.x > other.x {
            -1
        } else {
            0
        };
        result.1 = if self.y < other.y {
            1
        } else if self.y > other.y {
            -1
        } else {
            0
        };
        result.2 = if self.z < other.z {
            1
        } else if self.z > other.z {
            -1
        } else {
            0
        };
        result
    }

    fn apply_gravity_variation(&mut self, gravity: (i64, i64, i64)) {
        self.vx += gravity.0;
        self.vy += gravity.1;
        self.vz += gravity.2;
    }

    fn apply_velocity(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

    fn get_energy(&self) -> i64 {
        (self.x.abs() + self.y.abs() + self.z.abs())
            * (self.vx.abs() + self.vy.abs() + self.vz.abs())
    }
}

fn prepare_file(input: String) -> Vec<Moon> {
    let numbers = Regex::new(r"<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)").unwrap();
    let mut moons: Vec<Moon> = vec![];
    for cap in numbers.captures_iter(&input) {
        moons.push(Moon::new(
            cap["x"].parse().unwrap(),
            cap["y"].parse().unwrap(),
            cap["z"].parse().unwrap(),
        ))
    }
    moons
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut moons = prepare_file(fs::read_to_string(Path::new("./data/day12.txt"))?);
    let mut gravities = vec![];
    for (a, b) in (0..moons.len()).tuple_combinations() {
        gravities.push((a, b));
    }

    for _ in 0..1000 {
        for (a, b) in gravities.iter() {
            let variation = moons[*a].get_gravity_variation(&moons[*b]);
            moons[*a].apply_gravity_variation(variation);
            moons[*b].apply_gravity_variation((-variation.0, -variation.1, -variation.2))
        }

        for moon in moons.iter_mut() {
            moon.apply_velocity();
        }
    }

    let energy = moons.iter().fold(0, |acc, moon| acc + moon.get_energy());

    println!("Big moon energy! {}", energy);

    Ok(())
}

fn pgcd(a: i64, b: i64) -> i64 {
    if a % b == 0 {
        b
    } else {
        pgcd(b, a % b)
    }
}

fn ppcm(a: i64, b: i64) -> i64 {
    let x = std::cmp::max(a, b);
    let y = std::cmp::min(a, b);
    x * y / pgcd(x, y)
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut moons = prepare_file(fs::read_to_string(Path::new("./data/day12.txt"))?);
    let mut gravities = vec![];
    for (a, b) in (0..moons.len()).tuple_combinations() {
        gravities.push((a, b));
    }

    let mut hash_x: HashSet<String> = HashSet::new();
    let mut hash_y: HashSet<String> = HashSet::new();
    let mut hash_z: HashSet<String> = HashSet::new();

    let mut count_x: Option<i64> = None;
    let mut count_y: Option<i64> = None;
    let mut count_z: Option<i64> = None;

    let mut count = 0;

    while count_x.is_none() || count_y.is_none() || count_z.is_none() {
        for (a, b) in gravities.iter() {
            let variation = moons[*a].get_gravity_variation(&moons[*b]);
            moons[*a].apply_gravity_variation(variation);
            moons[*b].apply_gravity_variation((-variation.0, -variation.1, -variation.2))
        }

        let mut current_x = String::from("");
        let mut current_y = String::from("");
        let mut current_z = String::from("");

        for moon in moons.iter_mut() {
            moon.apply_velocity();
            current_x = format!("{}{}{}", current_x, moon.x, moon.vx);
            current_y = format!("{}{}{}", current_y, moon.y, moon.vy);
            current_z = format!("{}{}{}", current_z, moon.z, moon.vz);
        }

        if !hash_x.insert(current_x) && count_x.is_none() {
            count_x = Some(count);
        }

        if !hash_y.insert(current_y) && count_y.is_none() {
            count_y = Some(count);
        }

        if !hash_z.insert(current_z) && count_z.is_none() {
            count_z = Some(count);
        }

        count += 1;
    }

    let absolute_ppcm = ppcm(ppcm(count_x.unwrap(), count_y.unwrap()), count_z.unwrap());

    println!(
        "x loop: {}, y loop: {}, z loop: {}",
        count_x.unwrap_or(-1),
        count_y.unwrap_or(-1),
        count_z.unwrap_or(-1)
    );

    println!("Min time: {}", absolute_ppcm);

    Ok(())
}
