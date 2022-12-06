mod instruction;
mod parser;
mod ship;

use crate::ship::Ship;

struct Result {
    top_crates_9000: String,
    top_crates_9001: String,
}

fn main() {
    let input: &str = include_str!("../fixtures/input");
    let result: Result = day_five(input);
    println!("Top Stack 9000: {}", result.top_crates_9000);
    println!("Top Stack 9001: {}", result.top_crates_9001);
}

fn day_five(input: &str) -> Result {
    let mut ship: Ship = Ship::new(input);
    ship.move_crates_9000();
    let top_crates_9000: String = ship.top_crates();

    let mut ship: Ship = Ship::new(input);
    ship.move_crates_9001();
    let top_crates_9001: String = ship.top_crates();

    Result {
        top_crates_9000,
        top_crates_9001,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_returns_cmz() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_five(input);
        assert_eq!(result.top_crates_9000, "CMZ");
    }

    #[test]
    fn test_input_returns_mcd() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_five(input);
        assert_eq!(result.top_crates_9001, "MCD");
    }
}
