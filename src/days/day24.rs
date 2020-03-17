use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Empty,
    Bugs,
    Recursion,
}

trait GameOfLife {
    fn next_step(&self) -> Self;
    fn get_neighbor_value(&self, coordinate: (usize, usize), offset: (isize, isize)) -> Tile;
    fn get_score(&self) -> i64;
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
                        _ => {}
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
                    _ => unreachable!(),
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

    fn get_score(&self) -> i64 {
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

    let mut layout_code = area.get_score();

    while layouts.insert(layout_code) {
        area = GameOfLife::next_step(&area);
        layout_code = area.get_score();
    }

    println!("First repeating biodiversity: {}", layout_code);

    Ok(())
}

#[derive(Debug)]
enum Side {
    Left,
    Right,
    Down,
    Up,
}

trait RecursiveGameOfLife {
    fn new() -> Self;
    fn next_step_at_level(&self, recursion_level: isize, recursions: &HashMap<isize, Area>)
        -> Self;
    fn get_outer_infested_cells_count(&self, side: Side) -> usize;
    fn get_inner_infested_cells_count(&self, side: Side) -> usize;
    fn get_bugs_count_with_recurse(
        &self,
        coordinate: (usize, usize),
        offset: (isize, isize),
        current_recursion_level: isize,
        recursions: &HashMap<isize, Area>,
    ) -> usize;
}

impl RecursiveGameOfLife for Area {
    fn new() -> Self {
        let mut area = vec![];
        for _ in 0..5 {
            let mut line = vec![];
            for _ in 0..5 {
                line.push(Tile::Empty);
            }
            area.push(line);
        }

        area[2][2] = Tile::Recursion;

        area
    }

    fn next_step_at_level(
        &self,
        recursion_level: isize,
        recursions: &HashMap<isize, Area>,
    ) -> Self {
        let mut new_map = vec![];

        let offsets = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (line_no, line) in self.iter().enumerate() {
            let mut new_line = vec![];
            for (tile_no, tile) in line.iter().enumerate() {
                let mut infested_neighbors_count = 0;

                for offset in offsets.iter() {
                    infested_neighbors_count += self.get_bugs_count_with_recurse(
                        (line_no, tile_no),
                        *offset,
                        recursion_level,
                        &recursions,
                    );
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
                    Tile::Recursion => Tile::Recursion,
                });
            }
            new_map.push(new_line);
        }
        new_map
    }

    fn get_bugs_count_with_recurse(
        &self,
        coordinate: (usize, usize),
        offset: (isize, isize),
        current_recursion_level: isize,
        recursions: &HashMap<isize, Area>,
    ) -> usize {
        if coordinate.0 == 0 && offset.0 < 0
            || coordinate.1 == 0 && offset.1 < 0
            || coordinate.0 == 4 && offset.0 > 0
            || coordinate.1 == 4 && offset.1 > 0
        {
            let side = match offset {
                (-1, 0) => Side::Up,
                (1, 0) => Side::Down,
                (0, -1) => Side::Left,
                (0, 1) => Side::Right,
                _ => unreachable!(),
            };

            match recursions.get(&(current_recursion_level - 1)) {
                Some(outer_area) => outer_area.get_inner_infested_cells_count(side),
                None => 0,
            }
        } else {
            let line = self
                .get((coordinate.0 as isize + offset.0) as usize)
                .unwrap();
            match line
                .get((coordinate.1 as isize + offset.1) as usize)
                .unwrap()
            {
                Tile::Empty => 0,
                Tile::Bugs => 1,
                Tile::Recursion => {
                    let side = match offset {
                        (-1, 0) => Side::Down,
                        (1, 0) => Side::Up,
                        (0, -1) => Side::Right,
                        (0, 1) => Side::Left,
                        _ => unreachable!(),
                    };
                    match recursions.get(&(current_recursion_level + 1)) {
                        Some(inner_area) => inner_area.get_outer_infested_cells_count(side),
                        None => 0,
                    }
                }
            }
        }
    }

    fn get_outer_infested_cells_count(&self, side: Side) -> usize {
        use Side::*;
        match side {
            Up => self[0]
                .iter()
                .fold(0, |acc, elem| acc + if *elem == Tile::Bugs { 1 } else { 0 }),
            Down => self[4]
                .iter()
                .fold(0, |acc, elem| acc + if *elem == Tile::Bugs { 1 } else { 0 }),
            Left => self.iter().fold(0, |acc, elem| {
                acc + if elem[0] == Tile::Bugs { 1 } else { 0 }
            }),
            Right => self.iter().fold(0, |acc, elem| {
                acc + if elem[4] == Tile::Bugs { 1 } else { 0 }
            }),
        }
    }

    fn get_inner_infested_cells_count(&self, side: Side) -> usize {
        use Side::*;

        match side {
            Up => {
                if self[1][2] == Tile::Bugs {
                    1
                } else {
                    0
                }
            }
            Down => {
                if self[3][2] == Tile::Bugs {
                    1
                } else {
                    0
                }
            }
            Left => {
                if self[2][1] == Tile::Bugs {
                    1
                } else {
                    0
                }
            }
            Right => {
                if self[2][3] == Tile::Bugs {
                    1
                } else {
                    0
                }
            }
        }
    }
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let outer_upper_border_score = 31;
    let outer_left_border_score = 1_082_401;
    let outer_right_border_score = 17_318_416;
    let outer_down_border_score = 32_505_856;

    let inner_upper_border = 128;
    let inner_left_border = 2_048;
    let inner_right_border = 8_192;
    let inner_down_border = 131_072;

    let mut area = prepare_input(fs::read_to_string(Path::new("./data/day24.txt"))?);
    area[2][2] = Tile::Recursion;

    let mut outermost_level = 0;
    let mut innermost_level = 0;

    let mut recursive_layouts: HashMap<isize, Area> = HashMap::new();

    recursive_layouts.insert(0, area);

    for _ in 0..200 {
        let mut next_layout: HashMap<isize, Area> = HashMap::new();

        let innermost_area_score = recursive_layouts.get(&innermost_level).unwrap().get_score();
        let outermost_area_score = recursive_layouts.get(&outermost_level).unwrap().get_score();

        if (outermost_area_score & outer_upper_border_score > 0)
            || (outermost_area_score & outer_left_border_score > 0)
            || (outermost_area_score & outer_right_border_score > 0)
            || (outermost_area_score & outer_down_border_score > 0)
        {
            outermost_level -= 1;

            let new_outer_level = RecursiveGameOfLife::new();

            recursive_layouts.insert(outermost_level, new_outer_level);
        }

        if innermost_area_score & inner_upper_border > 0
            || innermost_area_score & inner_down_border > 0
            || innermost_area_score & inner_left_border > 0
            || innermost_area_score & inner_right_border > 0
        {
            innermost_level += 1;

            let new_inner_level = RecursiveGameOfLife::new();

            recursive_layouts.insert(innermost_level, new_inner_level);
        }

        for (depth, area) in recursive_layouts.iter() {
            let next_area = area.next_step_at_level(*depth, &recursive_layouts);

            next_layout.insert(*depth, next_area);
        }

        recursive_layouts = next_layout;
    }

    let total_bug_count = recursive_layouts.iter().fold(0, |acc, (_, area)| {
        acc + area.iter().fold(0, |acc, line| {
            acc + line
                .iter()
                .fold(0, |acc, tile| acc + if *tile == Tile::Bugs { 1 } else { 0 })
        })
    });

    println!("Total bug count after 200 minutes: {}", total_bug_count);

    Ok(())
}
