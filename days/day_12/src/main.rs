mod char;
mod hill;

use crate::hill::Hill;

struct Result {
    one: i32,
    two: i32,
}

fn main() {
    let input = include_str!("../fixtures/input");
    let result = day_twelve(input);
    println!("Part One: {}", result.one);
    println!("Part Two: {}", result.two);
}

fn day_twelve(input: &str) -> Result {
    let hill: Hill = Hill::new(input);

    let one: i32 = hill.shortest_path_to_end();
    let two: i32 = 0;

    Result { one, two }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_works() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_twelve(input);
        assert_eq!(result.one, 31);
    }
}
