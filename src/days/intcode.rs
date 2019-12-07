pub struct Intcode {
    memory: Vec<i32>,
    index: usize,
    inputs: Vec<i32>,
    pub output: i32,
    status: CompStatus,
}

type ParameterFlags = (bool, bool, bool);

#[derive(Debug, PartialEq)]
pub enum CompStatus {
    Running,
    Waiting,
    Halted,
    Error,
}

use self::CompStatus::*;

impl Intcode {
    pub fn new(memory: Vec<i32>) -> Intcode {
        Intcode {
            memory,
            index: 0,
            inputs: vec![],
            output: 0,
            status: Waiting,
        }
    }

    pub fn run(mut self) -> Intcode {
        while self.status == Running {
            self.status = self.next_op();
        }
        self
    }

    pub fn next_op(&mut self) -> CompStatus {
        let instruction = format!("{:05}", self.memory[self.index]);
        let mut instruction = instruction.chars();

        // The nth operation consume the element and the previous ones, nth(0) also shift the iterator
        let pos_mod_third = instruction.nth(0).unwrap() == '0';
        let pos_mod_second = instruction.nth(0).unwrap() == '0';
        let pos_mod_first = instruction.nth(0).unwrap() == '0';

        let parameter_flags = (pos_mod_first, pos_mod_second, pos_mod_third);

        let opcode: String = instruction.collect();

        // Possible refactoring: store functions in an hashmap with opcodes as keys
        match opcode.as_ref() {
            "01" => self.add(parameter_flags),
            "02" => self.mul(parameter_flags),
            "03" => self.use_input(parameter_flags),
            "04" => {
                let status = self.output(parameter_flags);
                println!("Output: {}", self.output);
                status
            }
            "05" => self.jump_if_true(parameter_flags),
            "06" => self.jump_if_false(parameter_flags),
            "07" => self.less_than(parameter_flags),
            "08" => self.equals(parameter_flags),
            "99" => Halted,
            _ => {
                println!("HALT AND CATCH FIRE");
                Error
            }
        }
    }

    pub fn add_input(mut self, input: i32) -> Self {
        self.inputs.reverse();
        self.inputs.push(input);
        self.inputs.reverse();

        if self.status == Waiting {
            self.status = Running;
        }

        self
    }

    fn prepare_op(
        &self,
        flags: ParameterFlags,
        op_len: usize,
        op_name: String,
    ) -> (Option<i32>, Option<i32>, Option<usize>) {
        if op_len == 3 && !flags.2 {
            panic!(
                "{}: Something went terribly wrong: param 3 is in immediate mode",
                op_name
            );
        }

        let first_value;
        let mut second_value = None;
        let mut store_index = None;

        let first_index = self.memory[self.index + 1];
        let second_index = if op_len >= 2 {
            self.memory[self.index + 2]
        } else {
            0
        };

        first_value = if flags.0 {
            Some(self.memory[first_index as usize])
        } else {
            Some(first_index)
        };

        if op_len >= 2 {
            second_value = if flags.1 {
                Some(self.memory[second_index as usize])
            } else {
                Some(second_index)
            };
        }

        if op_len == 3 {
            store_index = Some(self.memory[self.index + 3] as usize);
        }

        (first_value, second_value, store_index)
    }

    fn add(&mut self, flags: ParameterFlags) -> CompStatus {
        let (first_value, second_value, store_index) = self.prepare_op(flags, 3, "Add".to_string());

        self.memory[store_index.unwrap()] = first_value.unwrap() + second_value.unwrap();
        self.index += 4;
        Running
    }

    fn mul(&mut self, flags: ParameterFlags) -> CompStatus {
        let (first_value, second_value, store_index) = self.prepare_op(flags, 3, "Mul".to_string());

        self.memory[store_index.unwrap()] = first_value.unwrap() * second_value.unwrap();
        self.index += 4;
        Running
    }

    fn use_input(&mut self, flags: ParameterFlags) -> CompStatus {
        if self.inputs.is_empty() {
            return Waiting;
        }

        let new_input = self.inputs.pop().unwrap();

        if !flags.0 {
            panic!(
                "Input: Something went terribly wrong: param 1 is in immediate mode: {}",
                self.memory[self.index]
            );
        }

        let index = self.memory[self.index + 1] as usize;

        self.memory[index] = new_input;

        self.index += 2;
        Running
    }

    fn output(&mut self, flags: ParameterFlags) -> CompStatus {
        let index = self.memory[self.index + 1];

        self.output = if flags.0 {
            self.memory[index as usize]
        } else {
            index
        };

        self.index += 2;
        Running
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

        self.memory[store_index] = if first_value < second_value { 1 } else { 0 };

        self.index += 4;
        Running
    }

    fn equals(&mut self, flags: ParameterFlags) -> CompStatus {
        let (first_value, second_value, store_index) =
            self.prepare_op(flags, 3, "equals".to_string());
        let first_value = first_value.unwrap();
        let second_value = second_value.unwrap();
        let store_index = store_index.unwrap();

        self.memory[store_index] = if first_value == second_value { 1 } else { 0 };

        self.index += 4;
        Running
    }
}
