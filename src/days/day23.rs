use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::VecDeque;

use super::intcode;

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let nic = fs::read_to_string(Path::new("./data/day23.txt"))?;

    let mut network: Vec<intcode::Intcode> = vec![];
    let mut packets: Vec<VecDeque<(i64, i64)>> = vec![];
    for i in 0..50 {
        let mut comp = intcode::Intcode::new_with_path(nic.clone());
        comp = comp.run().add_input(i);
        packets.push(VecDeque::new());
        network.push(comp);
    }

    'main: loop {
        for (index, comp) in network.iter_mut().enumerate() {
            let status = comp.next_op();

            match status {
                intcode::CompStatus::Waiting => {
                    let packets = packets[index].pop_front().unwrap_or((-1, -1));
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
                if output[2] == 255 {
                    println!("Y value {} sent to packet 255", output[0]);
                    break 'main;
                }
                packets[output[2] as usize].push_back((output[1], output[0]));
            }
        }
    }
    Ok(())
}

#[derive(Debug, PartialEq)]
enum NodeStatus {
    Active,
    Waiting,
    Idle,
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let nic = fs::read_to_string(Path::new("./data/day23.txt"))?;

    let mut nat: Option<(i64, i64)> = None;
    let mut previous_y: Option<i64> = None;

    let mut network: Vec<intcode::Intcode> = vec![];
    let mut packets: Vec<VecDeque<(i64, i64)>> = vec![];

    let mut idlings: Vec<NodeStatus> = vec![];

    for i in 0..50 {
        let mut comp = intcode::Intcode::new_with_path(nic.clone());
        comp = comp.run().add_input(i);
        packets.push(VecDeque::new());
        network.push(comp);
        idlings.push(NodeStatus::Active);
    }

    loop {
        for (index, comp) in network.iter_mut().enumerate() {
            let status = comp.next_op();

            let output_len = comp.peek_outputs().len();

            if output_len > 0 {
                idlings[index] = NodeStatus::Active;
            }

            if output_len == 3 {
                let output = comp.get_outputs();
                if output[2] == 255 {
                    nat = Some((output[1], output[0]));
                } else {
                    packets[output[2] as usize].push_back((output[1], output[0]));
                    idlings[output[2] as usize] = NodeStatus::Active;
                }
            }

            match status {
                intcode::CompStatus::Waiting => {
                    let mut could_be_idling = output_len == 0;

                    if packets[index].is_empty() {
                        comp.add_input_borrowing(-1);
                        could_be_idling &= true;
                    } else {
                        let packet = packets[index].pop_front().unwrap();
                        comp.add_input_borrowing(packet.0);
                        comp.add_input_borrowing(packet.1);

                        could_be_idling = false;
                    }

                    idlings[index] = match idlings[index] {
                        NodeStatus::Active => {
                            if could_be_idling {
                                NodeStatus::Waiting
                            } else {
                                NodeStatus::Active
                            }
                        }
                        NodeStatus::Waiting => {
                            if could_be_idling {
                                NodeStatus::Idle
                            } else {
                                NodeStatus::Active
                            }
                        }
                        NodeStatus::Idle => {
                            if could_be_idling {
                                NodeStatus::Idle
                            } else {
                                NodeStatus::Active
                            }
                        }
                    };
                }
                intcode::CompStatus::Error | intcode::CompStatus::Halted => {
                    panic!("Should not happen")
                }
                _ => {}
            }
        }

        let is_truly_idling = idlings.iter().all(|curr| *curr == NodeStatus::Idle);

        if is_truly_idling {
            if let Some((x, y)) = nat {
                packets[0].push_back((x, y));
                idlings[0] = NodeStatus::Active;

                previous_y = if let Some(pre_y) = previous_y {
                    if pre_y == y {
                        println!("First repetition: {}", y);
                        break;
                    }
                    Some(y)
                } else {
                    Some(y)
                };
            }
        }
    }
    Ok(())
}
