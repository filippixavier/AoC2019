use std::error::Error;
use std::fs;
use std::path::Path;

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let current_input = fs::read_to_string(Path::new("./data/day16.txt"))?;
    let mut current_input = current_input
        .chars()
        .map(|elem| elem.to_digit(10).unwrap() as isize)
        .collect::<Vec<isize>>();

    for _ in 0..100 {
        let mut next_input = vec![];

        for i in 1..=current_input.len() / 2 {
            let mut next_value = 0;

            for (index, value) in current_input.iter().enumerate() {
                let position = ((index + 1) as f32 / i as f32).trunc();
                let pattern_value =
                    ((position * std::f32::consts::FRAC_PI_2).sin()).round() as isize;

                next_value += pattern_value * value;
            }

            next_input.push(next_value.abs() % 10);
        }

        let mut sum: isize = current_input
            .iter()
            .skip(current_input.len() / 2)
            .cloned()
            .sum::<isize>();

        for value in current_input.iter().skip(current_input.len() / 2) {
            next_input.push(sum % 10);
            sum -= value;
        }

        current_input = next_input;
    }

    let result = current_input
        .iter()
        .take(8)
        .fold(String::new(), |acc, elem| acc + &elem.to_string());

    println!("{}", result);
    Ok(())
}

// Solution HEAVILY influenced by this Reddit Thread: https://www.reddit.com/r/adventofcode/comments/ebf5cy/2019_day_16_part_2_understanding_how_to_come_up/
// Also remember using partial sum (https://github.com/enjmusic/aoc_2019/blob/master/aoc_16/src/main.rs => apply_fft)
pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let input = fs::read_to_string(Path::new("./data/day16.txt"))?;
    let message_offset = input
        .chars()
        .take(7)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let input: Vec<_> = input
        .chars()
        .map(|elem| elem.to_digit(10).unwrap() as isize)
        .collect();
    let initial_len = input.len();

    let mut input: Vec<_> = input
        .iter()
        .cycle()
        .take(initial_len * 10_000)
        .cloned()
        .collect();
    input = input
        .into_iter()
        .skip(message_offset)
        .collect::<Vec<isize>>();

    for _ in 0..100 {
        let mut next_input = vec![];

        let mut sum: isize = input.iter().sum::<isize>();

        for item in input {
            next_input.push(sum % 10);
            sum = (sum - item).abs();
        }

        input = next_input;
    }

    println!(
        "{}",
        input
            .into_iter()
            .take(8)
            .map(|x| x.to_string())
            .collect::<String>()
    );

    Ok(())
}
