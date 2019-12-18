use std::error::Error;
use std::fs;
use std::path::Path;

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let current_input = fs::read_to_string(Path::new("./data/day16.txt"))?;
    let mut current_input = current_input
        .chars()
        .map(|elem| elem.to_digit(10).unwrap() as isize)
        .collect::<Vec<isize>>();

    let base_pattern = vec![0, 1, 0, -1];
    let input_len = current_input.len();

    for _ in 0..100 {
        let mut new_input = vec![];

        for i in 1..=input_len {
            let mut next_value = 0;
            let mut current_pattern = vec![];
            for elem in base_pattern.iter() {
                for _ in 0..i {
                    current_pattern.push(*elem);
                }
            }

            for (index, value) in current_pattern
                .iter()
                .cycle()
                .skip(1)
                .take(input_len)
                .enumerate()
            {
                next_value += *value * current_input[index];
            }
            new_input.push(next_value.abs() % 10);
        }

        current_input = new_input;
    }

    let result = current_input
        .iter()
        .take(8)
        .fold(String::new(), |acc, elem| acc + &elem.to_string());

    println!("{}", result);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let current_input = fs::read_to_string(Path::new("./data/day16.txt"))?;
    let message_offset = current_input.chars().take(7).collect::<String>().parse::<usize>().unwrap();
    let mut current_input = current_input
        .chars()
        .map(|elem| elem.to_digit(10).unwrap() as isize)
        .collect::<Vec<isize>>();

    let base_pattern = vec![0, 1, 0, -1];
    let mut input_len = current_input.len();

    current_input = current_input.into_iter().cycle().take(1_000 * input_len).collect();
    input_len = current_input.len();

    for _ in 0..1 {
        let mut new_input = vec![];

        for i in 1..=input_len {
            let mut next_value = 0;
            let mut current_pattern = vec![];
            for elem in base_pattern.iter() {
                for _ in 0..i {
                    current_pattern.push(*elem);
                }
            }

            for (index, value) in current_pattern
                .iter()
                .cycle()
                .skip(1)
                .take(input_len)
                .enumerate()
            {
                next_value += *value * current_input[index];
            }
            new_input.push(next_value.abs() % 10);
        }

        current_input = new_input;
    }

    println!("{:?}", current_input);

    let result = current_input
        .iter()
        .skip(message_offset)
        .take(8)
        .fold(String::new(), |acc, elem| acc + &elem.to_string());

    println!("{}", result);
    Ok(())
}
