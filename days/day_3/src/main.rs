mod item;
mod priority;
mod rucksack;

use crate::rucksack::Rucksack;
use std::str::Split;

struct Result {
    priority_sum: i32,
}

fn main() {
    let input: &str = include_str!("../fixtures/input");
    let result: Result = day_three(input);
    println!("Priority Sum: {}", result.priority_sum);
}

fn day_three(input: &str) -> Result {
    let rucksack_inputs: Split<char> = input.split('\n');
    let mut priority_sum: i32 = 0;

    for rucksack_input in rucksack_inputs {
        if !rucksack_input.is_empty() {
            let rucksack = Rucksack::new(rucksack_input);
            priority_sum += rucksack.priority();
        }
    }

    Result { priority_sum }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_returns_157() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_three(input);
        assert_eq!(result.priority_sum, 157);
    }
}
