mod game;
mod hand;
mod score;

use crate::game::Game;
use crate::score::Score;
use std::str::Split;

struct Result {
    score: i32,
}

fn main() {
    let input = include_str!("../fixtures/input");
    let result = day_two(input);
    println!("Score: {}", result.score);
}

fn day_two(input: &str) -> Result {
    let game_inputs: Split<char> = input.split('\n');
    let mut score: i32 = 0;

    for game_input in game_inputs {
        if !game_input.is_empty() {
            let game = Game::new(game_input);
            score += game.score();
        }
    }

    Result { score }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_returns_15() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_two(input);
        assert_eq!(result.score, 15);
    }
}
