mod elf;
mod fruit;

use crate::elf::Elf;
use std::str::Split;

fn main() {
    let input = include_str!("../fixtures/input");
    day_one(input);
}

fn day_one(input: &str) -> i32 {
    let (_elves, max) = parse_elves(input);
    println!("{:?}", max);
    max
}

fn parse_elves(input: &str) -> (Vec<Elf>, i32) {
    let elf_inputs: Split<&str> = input.split("\n\n");
    let mut elves = Vec::new();
    let mut max: i32 = 0;

    for elf_input in elf_inputs {
        let elf: Elf = Elf::new(elf_input);
        if elf.calories > max {
            max = elf.calories;
        }
        elves.push(elf);
    }

    (elves, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_returns_24000() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: i32 = day_one(input);
        assert_eq!(result, 24000);
    }
}
