use std::error::Error;
use std::fs;
use std::path::Path;

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

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}

//2057 too low
