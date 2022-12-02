use crate::score::Score;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum HandKind {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    kind: HandKind,
}

impl Hand {
    pub fn new(kind: HandKind) -> Self {
        Hand { kind }
    }

    pub fn ties(&self, other: &Hand) -> bool {
        self.kind == other.kind
    }

    pub fn beats(&self, other: &Hand) -> bool {
        (self.kind == HandKind::Rock && other.kind == HandKind::Scissors)
            || (self.kind == HandKind::Paper && other.kind == HandKind::Rock)
            || (self.kind == HandKind::Scissors && other.kind == HandKind::Paper)
    }
}

impl FromStr for Hand {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, ParseError> {
        match input {
            "A" => Ok(Hand::new(HandKind::Rock)),
            "B" => Ok(Hand::new(HandKind::Paper)),
            "C" => Ok(Hand::new(HandKind::Scissors)),
            "X" => Ok(Hand::new(HandKind::Rock)),
            "Y" => Ok(Hand::new(HandKind::Paper)),
            "Z" => Ok(Hand::new(HandKind::Scissors)),
            _ => Err(ParseError {}),
        }
    }
}

impl Score for Hand {
    fn score(&self) -> i32 {
        match self.kind {
            HandKind::Rock => 1,
            HandKind::Paper => 2,
            HandKind::Scissors => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_is_rock() {
        let result = Hand::from_str("A");
        assert_eq!(
            result,
            Ok(Hand {
                kind: HandKind::Rock
            })
        );
    }

    #[test]
    fn b_is_paper() {
        let result = Hand::from_str("B");
        assert_eq!(
            result,
            Ok(Hand {
                kind: HandKind::Paper
            })
        );
    }

    #[test]
    fn c_is_scissors() {
        let result = Hand::from_str("C");
        assert_eq!(
            result,
            Ok(Hand {
                kind: HandKind::Scissors
            })
        );
    }

    #[test]
    fn x_is_rock() {
        let result = Hand::from_str("X");
        assert_eq!(
            result,
            Ok(Hand {
                kind: HandKind::Rock
            })
        );
    }

    #[test]
    fn y_is_paper() {
        let result = Hand::from_str("Y");
        assert_eq!(
            result,
            Ok(Hand {
                kind: HandKind::Paper
            })
        );
    }

    #[test]
    fn z_is_scissors() {
        let result = Hand::from_str("Z");
        assert_eq!(
            result,
            Ok(Hand {
                kind: HandKind::Scissors
            })
        );
    }

    #[test]
    fn errors_on_unknown_str() {
        let result = Hand::from_str("U");
        assert_eq!(result, Err(ParseError));
    }

    #[test]
    fn errors_on_empty_str() {
        let result = Hand::from_str("");
        assert_eq!(result, Err(ParseError));
    }
}
