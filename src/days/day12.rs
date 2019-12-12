use itertools::Itertools;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    vx: i32,
    vy: i32,
    vz: i32,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Moon {
            x,
            y,
            z,
            vx: 0,
            vy: 0,
            vz: 0,
        }
    }

    fn get_gravity_variation(&self, other: &Self) -> (i32, i32, i32) {
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

    fn apply_gravity_variation(&mut self, gravity: (i32, i32, i32)) {
        self.vx += gravity.0;
        self.vy += gravity.1;
        self.vz += gravity.2;
    }

    fn apply_velocity(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

    fn get_energy(&self) -> i32 {
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

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
