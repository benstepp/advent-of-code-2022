mod keep_away;
mod monkey;
mod operation;

use crate::keep_away::KeepAway;

struct Result {
    one: i32,
    two: i32,
}

fn main() {
    let input = include_str!("../fixtures/input");
    let result = day_eleven(input);
    println!("Monkey Business: {}", result.one);
    println!("two: {}", result.two);
}

fn day_eleven(input: &str) -> Result {
    let mut game: KeepAway = KeepAway::new(input);
    game.rounds(20);
    let one: i32 = game.monkey_business();
    let two: i32 = 0;

    Result { one, two }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_works() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_eleven(input);
        assert_eq!(result.one, 10605);
    }
}
