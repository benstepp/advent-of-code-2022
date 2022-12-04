mod elf;
mod pair;

use crate::pair::Pair;
use std::str::Split;

struct Result {
    completely_overlapping_pairs: i32,
    partially_overlapping_pairs: i32,
}

fn main() {
    let input: &str = include_str!("../fixtures/input");
    let result: Result = day_four(input);
    println!(
        "Completely Overlapping Pairs: {}",
        result.completely_overlapping_pairs
    );
    println!(
        "Partially Overlapping Pairs: {}",
        result.partially_overlapping_pairs
    );
}

fn day_four(input: &str) -> Result {
    let pair_inputs: Split<char> = input.split('\n');
    let mut completely_overlapping_pairs: i32 = 0;
    let mut partially_overlapping_pairs: i32 = 0;

    for pair_input in pair_inputs {
        if !pair_input.is_empty() {
            let pair = Pair::new(pair_input);
            if pair.completely_overlaps() {
                completely_overlapping_pairs += 1;
            }

            if pair.partially_overlaps() {
                partially_overlapping_pairs += 1;
            }
        }
    }

    Result {
        completely_overlapping_pairs,
        partially_overlapping_pairs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_returns_2_for_puzzle_one() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_four(input);
        assert_eq!(result.completely_overlapping_pairs, 2);
    }

    #[test]
    fn test_input_returns_4_for_puzzle_two() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_four(input);
        assert_eq!(result.partially_overlapping_pairs, 4);
    }
}
