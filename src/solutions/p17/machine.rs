use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum Instruction {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl Instruction {
    fn from_opcode(opcode: u8) -> Self {
        match opcode {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("Invalid opcode!"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct InstructionOperand {
    instruction: Instruction,
    operand: u8,
}

#[derive(Clone, Debug)]
pub struct Machine {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instruction_pointer: usize,
    raw_program: Vec<u8>,
    program: Vec<InstructionOperand>,
    output: Vec<u8>,
}

impl Machine {
    pub fn new(register_a: u64, register_b: u64, register_c: u64, program: Vec<u8>) -> Self {
        assert_eq!(program.len() % 2, 0);
        let raw_program = program.clone();
        let program: Vec<InstructionOperand> = program
            .into_iter()
            .chunks(2)
            .into_iter()
            .map(|lines| {
                let lines = lines.collect::<Vec<u8>>();
                let (opcode, operand) = (lines[0], lines[1]);
                let instruction = Instruction::from_opcode(opcode);
                InstructionOperand {
                    instruction,
                    operand,
                }
            })
            .collect();

        Machine {
            register_a,
            register_b,
            register_c,
            instruction_pointer: 0,
            raw_program,
            program,
            output: vec![],
        }
    }

    pub fn reset(&mut self, register_a: u64, register_b: u64, register_c: u64) {
        self.register_a = register_a;
        self.register_b = register_b;
        self.register_c = register_c;
        self.instruction_pointer = 0;
        self.output.clear();
    }

    pub fn get_program(&self) -> &Vec<u8> {
        &self.raw_program
    }
    pub fn get_output(&self) -> &Vec<u8> {
        &self.output
    }

    fn get_instruction_operand(&self) -> Option<InstructionOperand> {
        // Each instruction_operand in `self.program` is actually two lines
        // of assembly, so we need to divide the instruction pointer to account
        // for this when accessing an instruction-operand pair.
        if self.instruction_pointer / 2 < self.program.len() {
            Some(self.program[self.instruction_pointer / 2])
        } else {
            None
        }
    }

    fn get_literal_operand_value(&self, operand: u8) -> u64 {
        // This is a method to match the signature of `get_combo_operand_value`
        operand as u64
    }

    fn get_combo_operand_value(&self, operand: u8) -> u64 {
        match operand {
            0 | 1 | 2 | 3 => operand as u64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => panic!("reserved operand"),
            _ => panic!("invalid operand"),
        }
    }

    /// Executes the current instruction
    /// Returns true if successfully executed, otherwise, if we have reached
    /// the end of the program, we return false.
    pub fn execute_next(&mut self) -> bool {
        let InstructionOperand {
            instruction,
            operand,
        } = match self.get_instruction_operand() {
            Some(instruction_operand) => instruction_operand,
            None => return false,
        };

        // The `jnz`` (and perhaps more in the future) instruction requires that
        // the instruction pointer does not increment after executing it.
        let mut increment = true;

        match instruction {
            Instruction::Adv => {
                let operand_value = self.get_combo_operand_value(operand);
                let num = self.register_a;
                let denom = 1 << operand_value;
                self.register_a = num / denom;
            }
            Instruction::Bxl => {
                let operand_value = self.get_literal_operand_value(operand);
                self.register_b = self.register_b ^ operand_value;
            }
            Instruction::Bst => {
                let operand_value = self.get_combo_operand_value(operand);
                self.register_b = operand_value % 8;
            }
            Instruction::Jnz => {
                if self.register_a != 0 {
                    // My setup does not support instruction pointers at odd
                    // locations. Hopefully this won't be an issue!
                    let operand_value = self.get_literal_operand_value(operand);
                    assert!(operand_value % 2 != 1);
                    self.instruction_pointer = operand_value as usize;
                    increment = false;
                }
            }
            Instruction::Bxc => self.register_b = self.register_b ^ self.register_c,
            Instruction::Out => {
                let operand_value = self.get_combo_operand_value(operand);
                let output_value = (operand_value % 8) as u8;
                self.output.push(output_value)
            }
            Instruction::Bdv => {
                let operand_value = self.get_combo_operand_value(operand);
                let num = self.register_a;
                let denom = 1 << operand_value;
                self.register_b = num / denom;
            }
            Instruction::Cdv => {
                let operand_value = self.get_combo_operand_value(operand);
                let num = self.register_a;
                let denom = 1 << operand_value;
                self.register_c = num / denom;
            }
        }

        if increment {
            self.instruction_pointer += 2;
        }
        true
    }

    /// Continues executing the instructions until the program halts or a timeout
    /// is reached.
    const TIMEOUT_THRESHOLD: u64 = 100000;

    pub fn run(&mut self, debug: bool) {
        if debug {
            dbg!(&self);
        }
        for _ in 0..Self::TIMEOUT_THRESHOLD {
            let success = self.execute_next();
            if debug {
                dbg!(&self);
            }
            if !success {
                if debug {
                    println!("Program completed!")
                }
                return;
            }
        }
        panic!(
            "Machine reached timeout threshold: {}!",
            Self::TIMEOUT_THRESHOLD
        );
    }

    pub fn print_output(&self) {
        let output_str = self
            .output
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        println!("{output_str}");
    }
}
