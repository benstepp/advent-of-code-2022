use crate::score::Score;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandKind {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;

pub trait Hand {
    fn kind(&self) -> HandKind;

    fn ties(&self, other: &dyn Hand) -> bool {
        self.kind() == other.kind()
    }

    fn beats(&self, other: &dyn Hand) -> bool {
        (self.kind() == HandKind::Rock && other.kind() == HandKind::Scissors)
            || (self.kind() == HandKind::Paper && other.kind() == HandKind::Rock)
            || (self.kind() == HandKind::Scissors && other.kind() == HandKind::Paper)
    }
}

impl<T: Hand> Score for T {
    fn score(&self) -> i32 {
        match self.kind() {
            HandKind::Rock => 1,
            HandKind::Paper => 2,
            HandKind::Scissors => 3,
        }
    }
}
