use std::error::Error;
use std::io::{self};
use std::time::Instant;

mod days;

type Star = fn() -> Result<(), Box<dyn Error + 'static>>;

fn default_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}

fn main() {
    let mut buffer = String::new();
    let mut day: u32 = 1;
    println!("Please enter the day number: ");
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            // Must use trim_end or the line ending mess up with the parsing function
            day = buffer.trim_end().parse().unwrap_or(1);
            println!("Attempting to run day{}", day);
        }
        Err(error) => println!("Error: {}, defaulting to day1", error),
    }

    let first_star: Star;
    let second_star: Star;
    match day {
        1 => {
            first_star = days::day1::first_star;
            second_star = days::day1::second_star;
        }
        2 => {
            first_star = days::day2::first_star;
            second_star = days::day2::second_star;
        }
        3 => {
            first_star = days::day3::first_star;
            second_star = days::day3::second_star;
        }
        _ => {
            println!("Executing nothing");
            first_star = default_star;
            second_star = default_star;
        }
    }

    let now = Instant::now();
    match first_star() {
        Err(x) => {
            println!("Error: {:?}", x);
        }
        _ => {
            println!("First Star: Success!");
        }
    }
    match second_star() {
        Err(x) => {
            println!("Error {:?}", x);
        }
        _ => {
            println!("Second Star: Success!");
        }
    }
    let end = now.elapsed();
    println!("{}.{}", end.as_secs(), end.subsec_millis());
}