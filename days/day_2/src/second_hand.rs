use crate::hand::Hand;
use crate::hand::HandKind;
use crate::hand::ParseError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct SecondHand {
    kind: HandKind,
}

impl Hand for SecondHand {
    fn kind(&self) -> HandKind {
        self.kind
    }
}

impl SecondHand {
    pub fn new(kind: HandKind) -> Self {
        SecondHand { kind }
    }
}

impl FromStr for SecondHand {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, ParseError> {
        match input {
            "A" => Ok(Self::new(HandKind::Rock)),
            "B" => Ok(Self::new(HandKind::Paper)),
            "C" => Ok(Self::new(HandKind::Scissors)),
            "A X" => Ok(Self::new(HandKind::Scissors)),
            "A Y" => Ok(Self::new(HandKind::Rock)),
            "A Z" => Ok(Self::new(HandKind::Paper)),
            "B X" => Ok(Self::new(HandKind::Rock)),
            "B Y" => Ok(Self::new(HandKind::Paper)),
            "B Z" => Ok(Self::new(HandKind::Scissors)),
            "C X" => Ok(Self::new(HandKind::Paper)),
            "C Y" => Ok(Self::new(HandKind::Scissors)),
            "C Z" => Ok(Self::new(HandKind::Rock)),
            _ => Err(ParseError {}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_is_rock() {
        let result = SecondHand::from_str("A");
        assert_eq!(
            result,
            Ok(SecondHand {
                kind: HandKind::Rock
            })
        );
    }

    #[test]
    fn b_is_paper() {
        let result = SecondHand::from_str("B");
        assert_eq!(
            result,
            Ok(SecondHand {
                kind: HandKind::Paper
            })
        );
    }

    #[test]
    fn c_is_scissors() {
        let result = SecondHand::from_str("C");
        assert_eq!(
            result,
            Ok(SecondHand {
                kind: HandKind::Scissors
            })
        );
    }

    #[test]
    fn ax_is_scissors() {
        let result = SecondHand::from_str("A X");
        assert_eq!(
            result,
            Ok(SecondHand {
                kind: HandKind::Scissors
            })
        );
    }

    #[test]
    fn ay_is_rock() {
        let result = SecondHand::from_str("A Y");
        assert_eq!(
            result,
            Ok(SecondHand {
                kind: HandKind::Rock
            })
        );
    }

    #[test]
    fn az_is_paper() {
        let result = SecondHand::from_str("A Z");
        assert_eq!(
            result,
            Ok(SecondHand {
                kind: HandKind::Paper
            })
        );
    }

    #[test]
    fn bx_is_rock() {
        let result = SecondHand::from_str("B X");
        assert_eq!(
            result,
            Ok(SecondHand {
                kind: HandKind::Rock
            })
        );
    }

    #[test]
    fn by_is_paper() {
        let result = SecondHand::from_str("B Y");
        assert_eq!(
            result,
            Ok(SecondHand {
                kind: HandKind::Paper
            })
        );
    }

    #[test]
    fn bz_is_scissors() {
        let result = SecondHand::from_str("B Z");
        assert_eq!(
            result,
            Ok(SecondHand {
                kind: HandKind::Scissors
            })
        );
    }

    #[test]
    fn cx_is_rock() {
        let result = SecondHand::from_str("C X");
        assert_eq!(
            result,
            Ok(SecondHand {
                kind: HandKind::Paper
            })
        );
    }

    #[test]
    fn cy_is_paper() {
        let result = SecondHand::from_str("C Y");
        assert_eq!(
            result,
            Ok(SecondHand {
                kind: HandKind::Scissors
            })
        );
    }

    #[test]
    fn cz_is_scissors() {
        let result = SecondHand::from_str("C Z");
        assert_eq!(
            result,
            Ok(SecondHand {
                kind: HandKind::Rock
            })
        );
    }

    #[test]
    fn errors_on_unknown_str() {
        let result = SecondHand::from_str("U");
        assert_eq!(result, Err(ParseError));
    }

    #[test]
    fn errors_on_empty_str() {
        let result = SecondHand::from_str("");
        assert_eq!(result, Err(ParseError));
    }
}
