mod elf;
mod fruit;

use crate::elf::Elf;
use std::collections::BinaryHeap;
use std::str::Split;

struct Result {
    maximum: i32,
    top_three: i32,
}

fn main() {
    let input = include_str!("../fixtures/input");
    let result = day_one(input);
    println!("Maximum: {}", result.maximum);
    println!("Top Three: {}", result.top_three);
}

fn day_one(input: &str) -> Result {
    let elf_inputs: Split<&str> = input.split("\n\n");
    let mut elves = Vec::new();
    let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();

    for elf_input in elf_inputs {
        let elf: Elf = Elf::new(elf_input);
        max_heap.push(elf.calories);
        elves.push(elf);
    }

    let maximum: i32 = *max_heap.peek().unwrap();

    let top_three: i32 = {
        let mut result: i32 = 0;
        for _ in 0..3 {
            let val = max_heap.pop().unwrap();
            result += val;
        }
        result
    };

    Result { maximum, top_three }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_returns_24000_maximum() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_one(input);
        assert_eq!(result.maximum, 24000);
    }

    #[test]
    fn test_input_returns_45000_top_three() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_one(input);
        assert_eq!(result.top_three, 45000);
    }
}
