pub struct Intcode {
    memory: Vec<i64>,
    index: usize,
    inputs: Vec<i64>,
    relative_offset: i64,
    pub output: i64,
    pub outputs: Vec<i64>,
    pub status: CompStatus,
}

trait MemExpand {
    type Output;
    fn get(&mut self, index: usize) -> Self::Output;
    fn set(&mut self, index: usize, value: i64);
}

impl MemExpand for Vec<i64> {
    type Output = i64;
    fn get(&mut self, index: usize) -> Self::Output {
        if index >= self.len() {
            self.resize(index + 1, 0);
        }
        self[index]
    }
    fn set(&mut self, index: usize, value: i64) {
        if index >= self.len() {
            self.resize(index + 1, 0);
        }
        self[index] = value;
    }
}

type ParameterFlags = (ParameterMode, ParameterMode, ParameterMode);

#[derive(Debug, PartialEq)]
pub enum CompStatus {
    Running,
    Waiting,
    Halted,
    Error,
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

use self::CompStatus::*;
use self::ParameterMode::*;

impl Intcode {
    pub fn new(memory: Vec<i64>) -> Intcode {
        Intcode {
            memory,
            index: 0,
            inputs: vec![],
            output: 0,
            outputs: vec![],
            relative_offset: 0,
            status: Running,
        }
    }

    pub fn new_with_path(path: String) -> Intcode {
        let memory = prepare_memory(path);
        Intcode {
            memory,
            index: 0,
            inputs: vec![],
            output: 0,
            outputs: vec![],
            relative_offset: 0,
            status: Running,
        }
    }

    pub fn run(mut self) -> Self {
        while self.status == Running {
            self.status = self.next_op();
        }
        self
    }

    pub fn next_op(&mut self) -> CompStatus {
        fn convert_to_flag(input: char) -> ParameterMode {
            if input == '0' {
                Position
            } else if input == '1' {
                Immediate
            } else {
                Relative
            }
        }

        let instruction = format!("{:05}", self.memory[self.index]);
        let mut instruction = instruction.chars();

        // The nth operation consume the element and the previous ones, nth(0) also shift the iterator
        let parameter_flags = (
            convert_to_flag(instruction.nth(0).unwrap()),
            convert_to_flag(instruction.nth(0).unwrap()),
            convert_to_flag(instruction.nth(0).unwrap()),
        );

        // Set parameter flags in the right order
        let parameter_flags = (parameter_flags.2, parameter_flags.1, parameter_flags.0);

        let opcode: String = instruction.collect();

        // Possible refactoring: store functions in an hashmap with opcodes as keys
        match opcode.as_ref() {
            "01" => self.add(parameter_flags),
            "02" => self.mul(parameter_flags),
            "03" => self.use_input(parameter_flags),
            "04" => self.output(parameter_flags),
            "05" => self.jump_if_true(parameter_flags),
            "06" => self.jump_if_false(parameter_flags),
            "07" => self.less_than(parameter_flags),
            "08" => self.equals(parameter_flags),
            "09" => self.set_relative_offset(parameter_flags),
            "99" => Halted,
            _ => {
                println!("HALT AND CATCH FIRE");
                Error
            }
        }
    }

    pub fn add_input(mut self, input: i64) -> Self {
        self.inputs.reverse();
        self.inputs.push(input);
        self.inputs.reverse();

        if self.status == Waiting {
            self.status = Running;
        }

        self
    }

    fn prepare_op(
        &mut self,
        flags: ParameterFlags,
        op_len: usize,
        op_name: String,
    ) -> (Option<i64>, Option<i64>, Option<usize>) {
        if op_len == 3 && flags.2 == Immediate {
            panic!(
                "{}: Something went terribly wrong: param 3 is in immediate mode",
                op_name
            );
        }

        let first_value;
        let mut second_value = None;
        let mut store_index = None;

        let first_index = self.memory.get(self.index + 1);
        let second_index = if op_len >= 2 {
            self.memory.get(self.index + 2)
        } else {
            0
        };

        first_value = match flags.0 {
            Position => Some(self.memory.get(first_index as usize)),
            Immediate => Some(first_index),
            Relative => Some(
                self.memory
                    .get((first_index + self.relative_offset) as usize),
            ),
        };

        if op_len >= 2 {
            second_value = match flags.1 {
                Position => Some(self.memory.get(second_index as usize)),
                Immediate => Some(second_index),
                Relative => Some(
                    self.memory
                        .get((second_index + self.relative_offset) as usize),
                ),
            };
        }

        if op_len == 3 {
            store_index = match flags.2 {
                Position => Some(self.memory.get(self.index + 3) as usize),
                Relative => Some((self.memory.get(self.index + 3) + self.relative_offset) as usize),
                Immediate => {
                    unreachable!("Immediate in position 3 should NOT happen at this point")
                }
            };
        }

        (first_value, second_value, store_index)
    }

    fn add(&mut self, flags: ParameterFlags) -> CompStatus {
        let (first_value, second_value, store_index) = self.prepare_op(flags, 3, "Add".to_string());

        self.memory.set(
            store_index.unwrap(),
            first_value.unwrap() + second_value.unwrap(),
        );
        self.index += 4;
        Running
    }

    fn mul(&mut self, flags: ParameterFlags) -> CompStatus {
        let (first_value, second_value, store_index) = self.prepare_op(flags, 3, "Mul".to_string());

        self.memory.set(
            store_index.unwrap(),
            first_value.unwrap() * second_value.unwrap(),
        );
        self.index += 4;
        Running
    }

    fn use_input(&mut self, flags: ParameterFlags) -> CompStatus {
        if self.inputs.is_empty() {
            return Waiting;
        }

        let new_input = self.inputs.pop().unwrap();

        if flags.0 == Immediate {
            panic!(
                "Input: Something went terribly wrong: param 1 is in immediate mode: {}",
                self.memory[self.index]
            );
        }

        let index = if flags.0 == Position {
            self.memory.get(self.index + 1) as usize
        } else {
            (self.memory.get(self.index + 1) + self.relative_offset) as usize
        };

        self.memory.set(index, new_input);

        self.index += 2;
        Running
    }

    fn output(&mut self, flags: ParameterFlags) -> CompStatus {
        let index = self.memory[self.index + 1];

        self.output = match flags.0 {
            Position => self.memory[index as usize],
            Immediate => index,
            Relative => self.memory[(index + self.relative_offset) as usize],
        };

        // println!("{}", self.output);

        self.outputs.reverse();
        self.outputs.push(self.output);
        self.outputs.reverse();

        self.index += 2;
        Running
    }

    pub fn get_outputs(&mut self) -> Vec<i64> {
        let outs = self.outputs.clone();
        self.outputs = vec![];
        outs
    }

    fn jump_if_true(&mut self, flags: ParameterFlags) -> CompStatus {
        let (first_value, second_value, _) = self.prepare_op(flags, 2, "Jump if true".to_string());
        let first_value = first_value.unwrap();
        let second_value = second_value.unwrap();

        self.index = if first_value != 0 {
            second_value as usize
        } else {
            self.index + 3
        };
        Running
    }

    fn jump_if_false(&mut self, flags: ParameterFlags) -> CompStatus {
        let (first_value, second_value, _) = self.prepare_op(flags, 2, "Jump if false".to_string());
        let first_value = first_value.unwrap();
        let second_value = second_value.unwrap();

        self.index = if first_value == 0 {
            second_value as usize
        } else {
            self.index + 3
        };
        Running
    }

    fn less_than(&mut self, flags: ParameterFlags) -> CompStatus {
        let (first_value, second_value, store_index) =
            self.prepare_op(flags, 3, "Less than".to_string());
        let first_value = first_value.unwrap();
        let second_value = second_value.unwrap();
        let store_index = store_index.unwrap();

        self.memory
            .set(store_index, if first_value < second_value { 1 } else { 0 });

        self.index += 4;
        Running
    }

    fn equals(&mut self, flags: ParameterFlags) -> CompStatus {
        let (first_value, second_value, store_index) =
            self.prepare_op(flags, 3, "equals".to_string());
        let first_value = first_value.unwrap();
        let second_value = second_value.unwrap();
        let store_index = store_index.unwrap();

        self.memory
            .set(store_index, if first_value == second_value { 1 } else { 0 });

        self.index += 4;
        Running
    }

    fn set_relative_offset(&mut self, flags: ParameterFlags) -> CompStatus {
        let (first_value, _, _) = self.prepare_op(flags, 1, "Set relative offset".to_string());
        self.relative_offset += first_value.unwrap();
        self.index += 2;
        Running
    }
}

pub fn prepare_memory(input: String) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|x| (x.trim().parse::<i64>().unwrap_or(0)))
        .collect::<Vec<_>>()
}
