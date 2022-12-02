mod first_hand;
mod game;
mod hand;
mod score;
mod second_hand;

use crate::first_hand::FirstHand;
use crate::game::Game;
use crate::score::Score;
use crate::second_hand::SecondHand;
use std::str::Split;

struct Result {
    first_score: i32,
    second_score: i32,
}

fn main() {
    let input = include_str!("../fixtures/input");
    let result = day_two(input);
    println!("First Score: {}", result.first_score);
    println!("Second Score: {}", result.second_score);
}

fn day_two(input: &str) -> Result {
    let game_inputs: Split<char> = input.split('\n');
    let mut first_score: i32 = 0;
    let mut second_score: i32 = 0;

    for game_input in game_inputs {
        if !game_input.is_empty() {
            let game: Game<FirstHand> = Game::<FirstHand>::first_game(game_input);
            first_score += game.score();
            let game: Game<SecondHand> = Game::<SecondHand>::second_game(game_input);
            second_score += game.score();
        }
    }

    Result {
        first_score,
        second_score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_first_game_returns_15() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_two(input);
        assert_eq!(result.first_score, 15);
    }

    #[test]
    fn test_input_second_game_returns_12() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_two(input);
        assert_eq!(result.second_score, 12);
    }
}
