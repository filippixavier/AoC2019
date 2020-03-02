use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashSet;

extern crate num_bigint;
extern crate num_traits;

fn new_stack(stack: &mut [i32], _steps: i32) -> Vec<i32> {
    stack.reverse();
    (*stack).to_vec()
}

fn cut_n_card(stack: &mut [i32], steps: i32) -> Vec<i32> {
    let pivot = if steps >= 0 {
        steps as usize
    } else {
        (stack.len() as i32 + steps) as usize
    };

    let result = [&stack[pivot..], &stack[..pivot]].concat();

    result.to_vec()
}

fn deal_with_n(stack: &mut [i32], steps: i32) -> Vec<i32> {
    let mut shuffled_deck = stack.to_vec();

    let mut index = 0;
    let steps_usize = steps as usize;
    let len = stack.len();

    for elem in stack.iter() {
        shuffled_deck[index] = *elem;
        index = (index + steps_usize) % len;
    }
    shuffled_deck
}

type OP = (fn(&mut [i32], i32) -> Vec<i32>, i32);

fn prepare_file(input: String) -> Vec<OP> {
    let mut ops: Vec<OP> = vec![];

    for line in input.lines() {
        let splitted: Vec<String> = line.split(' ').map(|elem| elem.to_string()).collect();
        let attempt_parsing = splitted.last().unwrap().parse::<i32>();

        if let std::result::Result::Ok(value) = attempt_parsing {
            if splitted.len() == 2 {
                ops.push((cut_n_card, value));
            } else {
                ops.push((deal_with_n, value));
            }
        } else {
            ops.push((new_stack, 0));
        }
    }

    ops
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let ops = prepare_file(fs::read_to_string(Path::new("./data/day22.txt"))?);
    let mut deck: Vec<i32> = (0..10_007).collect();

    for (fun, steps) in ops {
        deck = fun(&mut deck, steps);
    }

    let result = deck.iter().position(|&x| x == 2019).unwrap_or(0);

    println!("{:?}", result);

    Ok(())
}

// Copied from https://www.csee.umbc.edu/~chang/cs203.s09/exteuclid.shtml, needed http://defeo.lu/in310/poly/euclide-bezout/ and https://www.mathraining.be/chapters/4?type=1&which=16 to understand it
// Inverse modulo is located in the 3 tuple
fn extended_euclid(a: i64, b:i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (d1, s1, t1) = extended_euclid(b, a%b);
        (d1, t1, s1 - (a / b) * t1) // See http://defeo.lu/in310/poly/euclide-bezout/ on recursive relation
    }
}

fn reverse_new_stack(stack_len: i64, arrival_pos: i64, _steps: i64) -> i64 {
    let pivot = stack_len / 2;
    if arrival_pos == pivot {
        pivot
    } else if arrival_pos < pivot {
        stack_len - arrival_pos - 1
    } else {
        -arrival_pos + stack_len - 1
    }
}

fn reverse_cut_n_card(stack_len: i64, arrival_pos: i64, steps: i64) -> i64 {
    let absolute_steps = if steps < 0 {
        stack_len + steps
    } else {
        steps
    };

    (arrival_pos + absolute_steps) % stack_len
}

fn reverse_deal_with_n(stack_len: i64, arrival_pos: i64, steps: i64) -> i64 {
    use num_bigint::BigInt;
    use num_traits::cast::ToPrimitive;

    let (_, _, mut reverse) = extended_euclid(stack_len, steps);
    while reverse < 0 {
        reverse += stack_len;
    }

    let a = BigInt::from(arrival_pos);
    let b = BigInt::from(reverse);
    let c = BigInt::from(stack_len);

    let x = (a * b) % c;

    x.to_i64().unwrap_or(0)
}

type ReverseOp = (fn(i64, i64, i64) -> i64, i64);

fn prepare_file_reverse(input: String) -> Vec<ReverseOp> {
    let mut ops: Vec<ReverseOp> = vec![];

    for line in input.lines() {
        let splitted: Vec<String> = line.split(' ').map(|elem| elem.to_string()).collect();
        let attempt_parsing = splitted.last().unwrap().parse::<i64>();

        if let std::result::Result::Ok(value) = attempt_parsing {
            if splitted.len() == 2 {
                ops.push((reverse_cut_n_card, value));
            } else {
                ops.push((reverse_deal_with_n, value));
            }
        } else {
            ops.push((reverse_new_stack, 0));
        }
    }
    ops.reverse();
    ops
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let ops = prepare_file_reverse(fs::read_to_string(Path::new("./data/day22.txt"))?);

    let mut already_solved: HashSet<i64> = HashSet::new();


    let mut resolved_index: Vec<i64> = Vec::new();

    let stack_len = 119_315_717_514_047i64;
    let total = 101_741_582_076_661i64;
    let mut index = 2020;
    let mut total_done = 0;

    already_solved.insert(index);
    
    for count in 0..total {
        resolved_index.push(index);
        
        for (fun, steps) in ops.iter() {
            index = fun(stack_len, index, *steps);
            if index < 0 {
                panic!("Ouch");
            }
        }

        if !already_solved.insert(index) {
            println!("{}", index);
            total_done = count;
            break;
        }
    }

    let start_point = resolved_index.iter().position(|x| *x == index).unwrap_or(0);
    resolved_index = resolved_index.into_iter().skip(start_point).collect();

    let remaining_shuffles = ((total - total_done) % resolved_index.len() as i64) as usize;

    println!("I still have {} shuffles to do", remaining_shuffles);

    println!("Result: {}", resolved_index[remaining_shuffles]);
    Ok(())
}

// 75628569713232 too high