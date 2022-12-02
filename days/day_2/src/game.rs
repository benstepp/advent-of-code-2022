use crate::first_hand::FirstHand;
use crate::hand::Hand;
use crate::score::Score;
use crate::second_hand::SecondHand;
use std::str::FromStr;

pub struct Game<T: Hand> {
    opponent: T,
    player: T,
}

impl<T: Hand> Game<T> {
    pub fn first_game(input: &str) -> Game<FirstHand> {
        let parts: Vec<&str> = input.split(' ').collect();
        let opponent = FirstHand::from_str(parts[0]).unwrap();
        let player = FirstHand::from_str(parts[1]).unwrap();

        Game { opponent, player }
    }

    pub fn second_game(input: &str) -> Game<SecondHand> {
        let parts: Vec<&str> = input.split(' ').collect();
        let opponent = SecondHand::from_str(parts[0]).unwrap();
        let player = SecondHand::from_str(input).unwrap();

        Game { opponent, player }
    }

    fn game_score(&self) -> i32 {
        if self.player.ties(&self.opponent) {
            3
        } else if self.player.beats(&self.opponent) {
            6
        } else {
            0
        }
    }
}

impl<T: Hand + Score> Score for Game<T> {
    fn score(&self) -> i32 {
        self.player.score() + self.game_score()
    }
}
