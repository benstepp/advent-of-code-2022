use crate::instruction::Instruction;
use crate::instruction::InstructionType;
use std::collections::HashMap;
use std::str::Split;

#[derive(Debug)]
pub struct Device {
    registers: HashMap<i32, i32>,
    instructions: Vec<Instruction>,
}

impl Device {
    pub fn new() -> Device {
        let instructions: Vec<Instruction> = Vec::new();
        let registers: HashMap<i32, i32> = HashMap::new();

        Device {
            instructions,
            registers,
        }
    }

    pub fn add_instructions(&mut self, input: &str) {
        let lines: Split<char> = input.split('\n');

        for line in lines {
            if line.is_empty() {
                continue;
            }
            let instruction: Instruction = Instruction::new(line);
            self.instructions.push(instruction);
        }

        self.compute_registers();
    }

    fn compute_registers(&mut self) {
        let mut clock: i32 = 0;
        let mut register: i32 = 1;

        for instruction in &self.instructions {
            for _ in 0..instruction.clocks {
                clock += 1;
                self.registers.insert(clock, register);
            }

            match instruction.instruction {
                InstructionType::Addx(val) => register += val,
                InstructionType::Noop => {}
            }
        }
    }

    pub fn signal_strength(&self, clock: i32) -> i32 {
        self.registers.get(&clock).unwrap() * clock
    }

    pub fn draw(&self) -> Vec<char> {
        let mut result: Vec<char> = Vec::with_capacity(240);
        let mut row: i32 = 0;

        for cycle in 1..=240 {
            let cursor: i32 = cycle - 1 - (40 * row);
            let register: &i32 = self.registers.get(&cycle).unwrap();

            if cursor_in_range_of_register(cursor, *register) {
                result.push('#');
            } else {
                result.push('.');
            }

            if cycle % 40 == 0 {
                row += 1
            }
        }

        result
    }
}

fn cursor_in_range_of_register(cursor: i32, register: i32) -> bool {
    (cursor == register - 1) || (cursor == register) || (cursor == register + 1)
}
