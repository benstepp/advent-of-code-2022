use crate::instruction::Instruction;
use crate::parser::Parser;
use crate::parser::ParserResult;
use std::str::RSplit;

#[derive(Debug)]
pub struct Ship {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl Ship {
    pub fn new(input: &str) -> Ship {
        let parsed: ParserResult = Parser::parse_input(input);
        let mut stacks: Vec<Vec<char>> = initialize_stacks(&parsed.index);
        build_stacks(&mut stacks, &parsed);
        let instructions: Vec<Instruction> = parse_instructions(&parsed.instructions);

        Ship {
            stacks,
            instructions,
        }
    }

    pub fn move_crates(&mut self) {
        for instruction in &self.instructions {
            let mut amount = instruction.number;

            while amount > 0 {
                if let Some(letter) = self.stacks[instruction.from as usize].pop() {
                    self.stacks[instruction.to as usize].push(letter);
                }
                amount -= 1;
            }
        }
    }

    pub fn top_crates(&self) -> String {
        let mut letters: Vec<char> = Vec::new();

        for stack in &self.stacks {
            letters.push(stack[stack.len() - 1]);
        }

        letters.iter().collect()
    }
}

fn initialize_stacks(index_mapping: &str) -> Vec<Vec<char>> {
    let mut max: i32 = 0;
    for letter in index_mapping.chars().rev() {
        if letter.is_numeric() {
            max = letter.to_string().parse::<i32>().unwrap();
            break;
        }
    }

    let mut stacks = Vec::new();
    while max > 0 {
        stacks.push(Vec::new());
        max -= 1;
    }

    stacks
}

fn build_stacks(stacks: &mut [Vec<char>], parsed: &ParserResult) {
    let stack_inputs: RSplit<char> = parsed.stacks.rsplit('\n');

    for row in stack_inputs {
        for (index, letter) in row.chars().enumerate() {
            if letter.is_alphabetic() {
                // lol
                let col: usize = parsed
                    .index
                    .chars()
                    .nth(index)
                    .unwrap()
                    .to_string()
                    .parse::<usize>()
                    .unwrap();
                stacks[col - 1].push(letter);
            }
        }
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let lines = input.split('\n');

    for line in lines {
        instructions.push(Instruction::new(line));
    }

    instructions
}
