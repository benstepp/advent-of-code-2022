mod elf;
mod pair;

use crate::pair::Pair;
use std::str::Split;

struct Result {
    overlapping_pairs: i32,
}

fn main() {
    let input: &str = include_str!("../fixtures/input");
    let result: Result = day_four(input);
    println!("Overlapping Pairs: {}", result.overlapping_pairs);
}

fn day_four(input: &str) -> Result {
    let overlapping_pairs: i32 = puzzle_one(input);

    Result { overlapping_pairs }
}

fn puzzle_one(input: &str) -> i32 {
    let pair_inputs: Split<char> = input.split('\n');
    let mut overlapping_pairs: i32 = 0;

    for pair_input in pair_inputs {
        if !pair_input.is_empty() {
            let pair = Pair::new(pair_input);
            if pair.overlaps() {
                overlapping_pairs += 1;
            }
        }
    }

    overlapping_pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_returns_2_for_puzzle_one() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_four(input);
        assert_eq!(result.overlapping_pairs, 2);
    }
}
