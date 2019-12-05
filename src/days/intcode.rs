pub struct Intcode {
    memory: Vec<i32>,
    index: usize,
    input: i32,
    output: i32,
}

type ParameterFlags = (bool, bool, bool);

impl Intcode {
    pub fn new(memory: Vec<i32>, input: i32) -> Intcode {
        Intcode {
            memory,
            index: 0,
            input,
            output: 0,
        }
    }
    pub fn next_op(&mut self) -> bool {
        let instruction = format!("{:05}", self.memory[self.index]);
        let mut instruction = instruction.chars();
        let pos_mod_first = instruction.nth(3).unwrap() == '0';
        let pos_mod_second = instruction.nth(2).unwrap() == '0';
        let pos_mod_third = instruction.nth(0).unwrap() == '0';

        let parameter_flags = (pos_mod_first, pos_mod_second, pos_mod_third);

        let opcode: String = instruction.skip(4).take(2).collect();

        match opcode.as_ref() {
            "01" => {
                self.add(parameter_flags);
                true
            }
            "02" => {
                self.mul(parameter_flags);
                true
            }
            "03" => {
                self.input(parameter_flags);
                true
            }
            "04" => {
                self.output(parameter_flags);
                true
            }
            "99" => false,
            _ => panic!("HALT AND CATCH FIRE"),
        }
    }

    fn add(&mut self, flags: ParameterFlags) {
        if !flags.2 {
            panic!("Add: Something went terribly wrong: param 3 is in immediate mode");
        }

        let store_index = self.memory[self.index + 3] as usize;
        let first_index = self.memory[self.index + 1];
        let second_index = self.memory[self.index + 2];

        let first_value = if flags.0 {
            self.memory[first_index as usize]
        } else {
            first_index
        };

        let second_value = if flags.1 {
            self.memory[second_index as usize]
        } else {
            second_index
        };

        self.memory[store_index] = first_value + second_value;
        self.index += 4;
    }

    fn mul(&mut self, flags: ParameterFlags) {
        if !flags.2 {
            panic!("Mul: Something went terribly wrong: param 3 is in immediate mode");
        }

        let store_index = self.memory[self.index + 3] as usize;
        let first_index = self.memory[self.index + 1];
        let second_index = self.memory[self.index + 2];

        let first_value = if flags.0 {
            self.memory[first_index as usize]
        } else {
            first_index
        };

        let second_value = if flags.1 {
            self.memory[second_index as usize]
        } else {
            second_index
        };

        self.memory[store_index] = first_value * second_value;
        self.index += 4;
    }

    fn input(&mut self, flags: ParameterFlags) {
        if !flags.0 {
            panic!("Input: Something went terribly wrong: param 1 is in immediate mode");
        }

        let index = self.memory[self.index + 1] as usize;

        self.memory[index] = self.input;

        self.index += 2;
    }

    fn output(&mut self, flags: ParameterFlags) {
        if !flags.0 {
            panic!("Input: Something went terribly wrong: param 1 is in immediate mode");
        }

        let index = self.memory[self.index + 1] as usize;

        self.output = self.memory[index];

        self.index += 2;
    }
}
