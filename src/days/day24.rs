use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Empty,
    Bugs,
}

trait GameOfLife {
    fn next_step(&self) -> Self;
    fn get_neighbor_value(&self, coordinate: (usize, usize), offset: (isize, isize)) -> Tile;
    fn get_code(&self) -> i64;
}

impl GameOfLife for Area {
    fn next_step(&self) -> Self {
        let mut new_map = vec![];

        let offsets = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (line_no, line) in self.iter().enumerate() {
            let mut new_line = vec![];
            for (tile_no, tile) in line.iter().enumerate() {
                let mut infested_neighbors_count = 0;

                for offset in offsets.iter() {
                    let neighbor_tile = self.get_neighbor_value((line_no, tile_no), *offset);

                    match neighbor_tile {
                        Tile::Empty => {}
                        Tile::Bugs => infested_neighbors_count += 1,
                    }
                }

                new_line.push(match tile {
                    Tile::Empty => {
                        if infested_neighbors_count == 1 || infested_neighbors_count == 2 {
                            Tile::Bugs
                        } else {
                            Tile::Empty
                        }
                    }
                    Tile::Bugs => {
                        if infested_neighbors_count == 1 {
                            Tile::Bugs
                        } else {
                            Tile::Empty
                        }
                    }
                });
            }
            new_map.push(new_line);
        }
        new_map
    }

    fn get_neighbor_value(&self, coordinate: (usize, usize), offset: (isize, isize)) -> Tile {
        let default = vec![];

        if coordinate.0 == 0 && offset.0 < 0 || coordinate.1 == 0 && offset.1 < 0 {
            Tile::Empty
        } else {
            let line = self
                .get((coordinate.0 as isize + offset.0) as usize)
                .unwrap_or(&default);
            *line
                .get((coordinate.1 as isize + offset.1) as usize)
                .unwrap_or(&Tile::Empty)
        }
    }

    fn get_code(&self) -> i64 {
        let mut code = 0i64;

        for (line_no, line) in self.iter().enumerate() {
            for (col_no, tile) in line.iter().enumerate() {
                if *tile == Tile::Bugs {
                    code += 2i64.pow((5 * line_no + col_no) as u32);
                }
            }
        }

        code
    }
}

type Area = Vec<Vec<Tile>>;

fn prepare_input(input: String) -> Area {
    let mut area: Area = vec![];

    for line in input.lines() {
        area.push(
            line.chars()
                .map(|elem| if elem == '.' { Tile::Empty } else { Tile::Bugs })
                .collect::<Vec<Tile>>(),
        );
    }

    area
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut area = prepare_input(fs::read_to_string(Path::new("./data/day24.txt"))?);
    let mut layouts: HashSet<i64> = HashSet::new();

    let mut layout_code = area.get_code();

    while layouts.insert(layout_code) {
        area = area.next_step();
        layout_code = area.get_code();
    }

    println!("First repeating biodiversity: {}", layout_code);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
