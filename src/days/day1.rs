use std::error::Error;
use std::fs;
use std::path::Path;

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    println!("{:?}", std::env::current_exe());
    println!(
        "{}",
        fs::read_to_string(Path::new("./data/day1.txt"))? // Yeah, it mostly depends on where you call the exec, but oh well...
            .split('\n')
            .map(|x| (x.trim().parse::<i32>().unwrap_or(0) / 3) - 2)
            .fold(0, |acc, x| x + acc)
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
