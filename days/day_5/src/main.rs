mod instruction;
mod parser;
mod ship;

use crate::ship::Ship;

struct Result {
    top_crates: String,
}

fn main() {
    let input: &str = include_str!("../fixtures/input");
    let result: Result = day_five(input);
    println!("Top Stack: {}", result.top_crates);
}

fn day_five(input: &str) -> Result {
    let mut ship: Ship = Ship::new(input);
    ship.move_crates();
    let top_crates: String = ship.top_crates();
    Result { top_crates }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_returns_cmz() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_five(input);
        assert_eq!(result.top_crates, "CMZ");
    }
}
