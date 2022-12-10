mod parser;
mod quadcopter;
mod scenic_score;

use crate::parser::parse;
use crate::quadcopter::find_visible_trees;
use crate::scenic_score::max_scenic_score;

struct Result {
    visible_trees: i32,
    scenic_score: i32,
}

fn main() {
    let input: &str = include_str!("../fixtures/input");
    let result: Result = day_eight(input);
    println!("Visible Trees: {}", result.visible_trees);
    println!("Max Scenic Score: {}", result.scenic_score);
}

fn day_eight(input: &str) -> Result {
    let matrix: Vec<Vec<i32>> = parse(input);
    let visible_trees: i32 = find_visible_trees(matrix.clone());
    let scenic_score: i32 = max_scenic_score(matrix);
    Result {
        visible_trees,
        scenic_score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_works() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_eight(input);
        assert_eq!(result.visible_trees, 21);
        assert_eq!(result.scenic_score, 8);
    }
}
