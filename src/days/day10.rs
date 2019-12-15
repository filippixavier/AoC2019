use std::error::Error;
use std::fs;
use std::path::Path;

use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

fn prepare_file(input: String) -> Vec<Point> {
    input
        .trim()
        .split('\n')
        .enumerate()
        .fold(vec![], |mut acc: Vec<Point>, (y, line)| {
            for (x, character) in line.chars().enumerate() {
                if let '#' = character {
                    acc.push(Point { x, y });
                };
            }
            acc
        })
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let map = prepare_file(fs::read_to_string(Path::new("./data/day10.txt"))?);
    let mut best_detection = 0;
    let mut best_spot = Point { x: 0, y: 0 };

    for asteroid_a in &map {
        let mut equations = vec![];
        for asteroid_b in &map {
            if asteroid_b != asteroid_a {
                let h = match asteroid_b.x.cmp(&asteroid_a.x) {
                    Ordering::Less => 'l',
                    Ordering::Equal => 'e',
                    Ordering::Greater => 'r',
                };

                let v = match asteroid_b.y.cmp(&asteroid_a.y) {
                    Ordering::Less => 'u',
                    Ordering::Equal => 'e',
                    Ordering::Greater => 'd',
                };

                let equ = (asteroid_b.y as f32 - asteroid_a.y as f32)
                    / (asteroid_b.x as f32 - asteroid_a.x as f32);

                equations.push(format!("{}{}{}", h, v, equ));
            }
        }
        equations.sort_unstable();
        equations.dedup();
        best_detection = std::cmp::max(best_detection, equations.len());
        best_spot = if best_detection == equations.len() {
            *asteroid_a
        } else {
            best_spot
        }
    }

    println!("Max visibility is:{}, on {:?}", best_detection, best_spot);

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct Line {
    x: i32,
    y: i32,
    destination: Point,
    angle: f32,
    dist: i32,
    equation: String,
}

impl Line {
    fn new(origin: Point, destination: Point) -> Self {
        let x = destination.x as i32 - origin.x as i32;
        let y = destination.y as i32 - origin.y as i32;
        let mut angle = (-y as f32 / ((x.pow(2) + y.pow(2)) as f32).sqrt()).acos();
        angle = (angle * 1000.0).round() / 1000.0;
        if x < 0 {
            angle = 2.0 * std::f32::consts::PI - angle;
        }

        if angle.is_nan() {
            println!("{:?}, {:?}", origin, destination);
        }

        // Kinda hacking my way around float as working with angles introduce way to much approximation
        let h = match destination.x.cmp(&origin.x) {
            Ordering::Less => 'l',
            Ordering::Equal => 'e',
            Ordering::Greater => 'r',
        };

        let v = match destination.y.cmp(&origin.y) {
            Ordering::Less => 'u',
            Ordering::Equal => 'e',
            Ordering::Greater => 'd',
        };
        let equ =
            (destination.y as f32 - origin.y as f32) / (destination.x as f32 - origin.x as f32);
        Line {
            x,
            y,
            destination,
            angle,
            dist: x.abs() + y.abs(),
            equation: format!("{}{}{}", h, v, equ),
        }
    }
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut asteroid_maps = prepare_file(fs::read_to_string(Path::new("./data/day10.txt"))?);

    let best_spot = Point { x: 23, y: 19 }; // From previous star

    asteroid_maps = asteroid_maps
        .into_iter()
        .filter(|x| *x != best_spot)
        .collect();

    let mut lines: Vec<Line> = asteroid_maps
        .iter()
        .map(|asteroid| Line::new(best_spot, *asteroid))
        .collect();

    lines.sort_by(|a, b| {
        if (a.angle - b.angle).abs() < std::f32::EPSILON {
            a.dist.cmp(&b.dist)
        } else {
            a.angle.partial_cmp(&b.angle).unwrap()
        }
    });

    let mut count = 0;
    let destination;

    loop {
        let mut removed = lines.clone();
        removed.dedup_by(|a, b| a.equation == b.equation);

        if count + removed.len() >= 200 {
            destination = removed[199 - count].destination;
            break;
        } else {
            count += removed.len();
            lines = lines
                .into_iter()
                .filter(|elem| !removed.iter().any(|x| x == elem))
                .collect();
        }
    }

    println!(
        "200th destroyed: {:?}, answer is: {}",
        destination,
        destination.x * 100 + destination.y
    );

    Ok(())
}
