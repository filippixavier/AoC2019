use std::error::Error;
use std::fs;
use std::path::Path;

fn prepare_file(input: String, stack_len: i128) -> (i128, i128) {
    let mut coef = (1, 0);

    for line in input.lines() {
        let splitted: Vec<String> = line.split(' ').map(|elem| elem.to_string()).collect();
        let attempt_parsing = splitted.last().unwrap().parse::<i128>();

        if let std::result::Result::Ok(value) = attempt_parsing {
            if splitted.len() == 2 {
                coef.1 -= value;
            } else {
                coef.0 *= value;
                coef.1 *= value;
            }
        } else {
            coef.0 = -coef.0;
            coef.1 = -coef.1 - 1;
        }

        coef.0 %= stack_len;
        coef.1 %= stack_len;
    }

    coef
}

// Copied from https://www.csee.umbc.edu/~chang/cs203.s09/exteuclid.shtml, needed http://defeo.lu/in310/poly/euclide-bezout/ and https://www.mathraining.be/chapters/4?type=1&which=16 to understand it
// Inverse modulo is located in the 3 tuple
fn extended_euclid(max: i128, min:i128) -> (i128, i128, i128) {
    if min == 0 {
        (max, 1, 0)
    } else {
        let (d1, s1, t1) = extended_euclid(min, max%min);
        (d1, t1, s1 - (max / min) * t1) // See http://defeo.lu/in310/poly/euclide-bezout/ on recursive relation
    }
}

// https://en.wikipedia.org/wiki/Exponentiation_by_squaring, provided me wrong results because of the way I initialy handled odd numbered powers, couldn't have guessed without code from here https://github.com/enjmusic/aoc_2019/blob/master/aoc_22/src/main.rs
fn modular_power(base: i128, power: i128, modulo: i128) -> i128 {
    let mut total = base % modulo;
    let mut current_pow = power;
    let mut temp = 1;

    while current_pow > 1 {
        if current_pow % 2 == 1 {
            temp = (temp * total) % modulo;
            current_pow -= 1;
        }
        total = total.pow(2) % modulo;
        current_pow /= 2;
    }
    (total * temp) % modulo
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let stack_len = 10_007;
    let coef = prepare_file(fs::read_to_string(Path::new("./data/day22.txt"))?, stack_len);

    let mut result = (2019 * coef.0 + coef.1) % stack_len;

    if result < 0 {
        result += stack_len;
    }

    println!("From position 2019 to {}", result);

    Ok(())
}


pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let stack_len = 119_315_717_514_047_i128;
    let num_of_computations = 101_741_582_076_661_i128;

    let (factor, constant) = prepare_file(fs::read_to_string(Path::new("./data/day22.txt"))?, stack_len);

    let (_, _, denominator_inverse) = extended_euclid(stack_len, 1 - factor);

    let total_multiplier = modular_power(factor, num_of_computations, stack_len);

    // let total_offset = constant * (1 - total_multiplier) / (1 - factor); // Because division doesn't really works with modulo?

    let total_offset = (constant * (((1 - total_multiplier) * denominator_inverse) % stack_len)) % stack_len;

    let (_, _, reverse_multiplier) = extended_euclid(stack_len, total_multiplier); // Note: According to solution, I could also find any reverse by calculing a^b where b is stack_len - 2, since stack_len is prime, any a^(stack_len-1) = 1 mod stack_len

    let mut result = (2020 - total_offset) % stack_len;
    result = (result * reverse_multiplier) % stack_len;

    if result < 0 {
        result += stack_len;
    }

    println!("{}", result);

    Ok(())
}