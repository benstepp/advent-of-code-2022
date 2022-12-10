mod parser;
mod quadcopter;

use crate::parser::parse;
use crate::quadcopter::find_visible_trees;

struct Result {
    visible_trees: i32,
}

fn main() {
    let input: &str = include_str!("../fixtures/input");
    let result: Result = day_eight(input);
    println!("Visible Trees: {}", result.visible_trees);
}

fn day_eight(input: &str) -> Result {
    let matrix: Vec<Vec<i32>> = parse(input);
    let visible_trees: i32 = find_visible_trees(matrix);
    Result { visible_trees }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_works() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_eight(input);
        assert_eq!(result.visible_trees, 21);
    }
}
