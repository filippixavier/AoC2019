use std::error::Error;
use std::fs;
use std::path::Path;

use super::intcode;

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let nic = fs::read_to_string(Path::new("./data/day23.txt"))?;

    let mut network: Vec<intcode::Intcode> = vec![];
    let mut packets: Vec<Vec<(i64, i64)>> = vec![];
    for i in 0..50 {
        let mut comp = intcode::Intcode::new_with_path(nic.clone());
        comp = comp.run().add_input(i);
        packets.push(vec![]);
        network.push(comp);
    }

    'main: loop {
        for (index, comp) in network.iter_mut().enumerate() {
            let status = comp.next_op();

            match status {
                intcode::CompStatus::Waiting => {
                    let packets = packets[index].pop().unwrap_or((-1, -1));
                    comp.add_input_borrowing(packets.0);
                    if packets.0 != -1 {
                        comp.add_input_borrowing(packets.1);
                    }
                }
                intcode::CompStatus::Error | intcode::CompStatus::Halted => {
                    panic!("Should not happen")
                }
                _ => {}
            }

            if comp.peek_outputs().len() == 3 {
                let output = comp.get_outputs();
                println!("{:?}", output);
                if output[2] == 255 {
                    println!("Y value {} sent to packet 255", output[0]);
                    break 'main;
                }
                packets[output[2] as usize].push((output[1], output[0]));
            }
        }
    }
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
