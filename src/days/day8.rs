use std::error::Error;
use std::fs;
use std::path::Path;

const WIDTH: usize = 25;
const PAGE_SIZE: usize = 25 * 6;

fn prepare_file(input: String) -> Vec<u32> {
    input
        .trim()
        .chars()
        .map(|c| (c.to_digit(10).unwrap_or(0)))
        .collect::<Vec<_>>()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let img = prepare_file(fs::read_to_string(Path::new("./data/day8.txt"))?);

    let mut min_num_of_zero: Option<usize> = None;
    let mut checksum = 0;

    for page in img.chunks(PAGE_SIZE) {
        let (zero_count, one_count, two_count) = page.iter().fold((0, 0, 0), |acc, elem| {
            if *elem == 0 {
                (acc.0 + 1, acc.1, acc.2)
            } else if *elem == 1 {
                (acc.0, acc.1 + 1, acc.2)
            } else if *elem == 2 {
                (acc.0, acc.1, acc.2 + 1)
            } else {
                acc
            }
        });
        min_num_of_zero = if let Some(previous_min) = min_num_of_zero {
            if previous_min > zero_count {
                checksum = one_count * two_count;
                Some(zero_count)
            } else {
                Some(previous_min)
            }
        } else {
            checksum = one_count * two_count;
            Some(zero_count)
        }
    }

    println!("Checksum: {}", checksum);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let img = prepare_file(fs::read_to_string(Path::new("./data/day8.txt"))?);
    let mut result: Vec<u32> = vec![2; PAGE_SIZE];

    for page in img.chunks(PAGE_SIZE) {
        for (index, elem) in page.iter().enumerate() {
            result[index] = if result[index] == 2 {
                *elem
            } else {
                result[index]
            }
        }
    }

    println!("Printing result: ");

    for (index, value) in result.iter().enumerate() {
        let character = if *value == 0 { "\u{25A1}" } else { "■" };
        if index % WIDTH == 0 {
            println!();
        }
        print!("{}", character);
    }
    println!();

    Ok(())
}
