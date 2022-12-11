mod rope_bridge;
mod step;

use crate::rope_bridge::RopeBridge;

struct Result {
    unique_tail_coords: i32,
    longer: i32,
}

fn main() {
    let input: &str = include_str!("../fixtures/input");
    let result: Result = day_nine(input);
    println!("Unique Tail Coords: {}", result.unique_tail_coords);
    println!("Unique Tail Coords: {}", result.longer);
}

fn day_nine(input: &str) -> Result {
    let mut rope_bridge = RopeBridge::new();
    rope_bridge.add_steps(input);

    let unique_tail_coords: i32 = rope_bridge.unique_tail_locations(1);
    let longer: i32 = rope_bridge.unique_tail_locations(9);
    Result {
        unique_tail_coords,
        longer,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_works() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_nine(input);
        assert_eq!(result.unique_tail_coords, 13);
        assert_eq!(result.longer, 1);
    }

    #[test]
    fn bigger_test_input_works() {
        let input: &str = include_str!("../fixtures/test_input_2");
        let result: Result = day_nine(input);
        assert_eq!(result.longer, 36);
    }
}
