use std::error::Error;

use regex::Regex;

const START: u32 = 134_792;
const END: u32 = 675_810;

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let following_number = Regex::new(r"11|22|33|44|55|66|77|88|99").unwrap(); //Crate can't handle backreferences...
    let mut counter = 0;

    for i in START..END {
        let num_in_string = i.to_string();
        if following_number.is_match(&num_in_string) {
            let mut reordered: Vec<char> = num_in_string.clone().chars().collect();
            reordered.sort_by(|a, b| a.cmp(b));
            let reordered = reordered.iter().collect::<String>();

            if reordered == num_in_string {
                counter += 1;
            }
        }
    }

    println!("Number of passwords: {}", counter);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let following_number = Regex::new(r"([^1]|^)11([^1]|$)|([^2]|^)22([^2]|$)|([^3]|^)33([^3]|$)|([^4]|^)44([^4]|$)|([^5]|^)55([^5]|$)|([^6]|^)66([^6]|$)|([^7]|^)77([^7]|$)|([^8]|^)88([^8]|$)|([^9]|^)99([^9]|$)").unwrap(); //Crate can't handle backreferences...
    let mut counter = 0;

    for i in START..END {
        let num_in_string = i.to_string();
        if following_number.is_match(&num_in_string) {
            let mut reordered: Vec<char> = num_in_string.clone().chars().collect();
            reordered.sort_by(|a, b| a.cmp(b));
            let reordered = reordered.iter().collect::<String>();

            if reordered == num_in_string {
                counter += 1;
            }
        }
    }

    println!("Number of passwords: {}", counter);

    Ok(())
}
