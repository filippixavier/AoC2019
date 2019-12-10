use std::error::Error;
use std::fs;
use std::path::Path;

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

    for asteroid_a in &map {
        let mut equations = vec![];
        for asteroid_b in &map {
            if asteroid_b != asteroid_a {
                let h = if asteroid_b.x < asteroid_a.x {
                    "l"
                } else if asteroid_b.x > asteroid_a.x {
                    "r"
                } else {
                    "e"
                };
                let v = if asteroid_b.y < asteroid_a.y {
                    "u"
                } else if asteroid_b.y > asteroid_a.y {
                    "d"
                } else {
                    "e"
                };
                let equ = (asteroid_b.y as f32 - asteroid_a.y as f32)
                    / (asteroid_b.x as f32 - asteroid_a.x as f32);

                equations.push(format!("{}{}{}", h, v, equ));
            }
        }
        equations.sort_unstable();
        equations.dedup();
        best_detection = std::cmp::max(best_detection, equations.len());
    }

    println!("Max visibility is:{}", best_detection);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
