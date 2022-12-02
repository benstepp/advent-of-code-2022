use crate::hand::Hand;
use crate::score::Score;
use std::str::FromStr;

pub struct Game {
    opponent: Hand,
    player: Hand,
}

impl Game {
    pub fn new(input: &str) -> Game {
        let parts: Vec<&str> = input.split(' ').collect();
        let opponent = Hand::from_str(parts[0]).unwrap();
        let player = Hand::from_str(parts[1]).unwrap();

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

impl Score for Game {
    fn score(&self) -> i32 {
        self.player.score() + self.game_score()
    }
}
