#[derive(Debug)]
pub enum InstructionType {
    Addx(i32),
    Noop,
}

#[derive(Debug)]
pub struct Instruction {
    pub instruction: InstructionType,
    pub clocks: i32,
}

impl Instruction {
    pub fn new(input: &str) -> Instruction {
        let parts: Vec<&str> = input.split(' ').collect::<Vec<&str>>();
        match parts.len() {
            2 => make_addx(parts),
            1 => make_noop(),
            _ => {
                panic!("invalid instruction type")
            }
        }
    }
}

fn make_addx(parts: Vec<&str>) -> Instruction {
    let value = parts[1].parse::<i32>().unwrap();
    Instruction {
        instruction: InstructionType::Addx(value),
        clocks: 2,
    }
}

fn make_noop() -> Instruction {
    Instruction {
        instruction: InstructionType::Noop,
        clocks: 1,
    }
}
