mod elf;
mod group;
mod item;
mod priority;
mod rucksack;

use crate::group::Group;
use crate::rucksack::Rucksack;
use std::slice::Chunks;
use std::str::Split;

struct Result {
    priority_sum: i32,
    badge_sum: i32,
}

fn main() {
    let input: &str = include_str!("../fixtures/input");
    let result: Result = day_three(input);
    println!("Priority Sum: {}", result.priority_sum);
    println!("Badge Sum: {}", result.badge_sum);
}

fn day_three(input: &str) -> Result {
    let priority_sum: i32 = puzzle_one(input);
    let badge_sum: i32 = puzzle_two(input);

    Result {
        priority_sum,
        badge_sum,
    }
}

fn puzzle_one(input: &str) -> i32 {
    let rucksack_inputs: Split<char> = input.split('\n');
    let mut priority_sum: i32 = 0;

    for rucksack_input in rucksack_inputs {
        if !rucksack_input.is_empty() {
            let rucksack = Rucksack::new(rucksack_input);
            priority_sum += rucksack.priority();
        }
    }

    priority_sum
}

fn puzzle_two(input: &str) -> i32 {
    let lines: Vec<&str> = input.split('\n').collect::<Vec<&str>>();
    let chunks: Chunks<&str> = lines.chunks(3);
    let mut badge_sum: i32 = 0;

    for chunk in chunks {
        if chunk.len() == 3 {
            let group: Group = Group::new(chunk.to_vec());
            badge_sum += group.badge_priority();
        }
    }

    badge_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_returns_157_for_puzzle_one() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_three(input);
        assert_eq!(result.priority_sum, 157);
    }
    #[test]

    fn test_input_returns_70_for_puzzle_two() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_three(input);
        assert_eq!(result.badge_sum, 70);
    }
}
