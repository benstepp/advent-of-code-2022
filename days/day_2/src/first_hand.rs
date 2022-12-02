use crate::hand::Hand;
use crate::hand::HandKind;
use crate::hand::ParseError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct FirstHand {
    kind: HandKind,
}

impl Hand for FirstHand {
    fn kind(&self) -> HandKind {
        self.kind
    }
}

impl FirstHand {
    pub fn new(kind: HandKind) -> Self {
        FirstHand { kind }
    }
}

impl FromStr for FirstHand {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, ParseError> {
        match input {
            "A" => Ok(Self::new(HandKind::Rock)),
            "B" => Ok(Self::new(HandKind::Paper)),
            "C" => Ok(Self::new(HandKind::Scissors)),
            "X" => Ok(Self::new(HandKind::Rock)),
            "Y" => Ok(Self::new(HandKind::Paper)),
            "Z" => Ok(Self::new(HandKind::Scissors)),
            _ => Err(ParseError {}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_is_rock() {
        let result = FirstHand::from_str("A");
        assert_eq!(
            result,
            Ok(FirstHand {
                kind: HandKind::Rock
            })
        );
    }

    #[test]
    fn b_is_paper() {
        let result = FirstHand::from_str("B");
        assert_eq!(
            result,
            Ok(FirstHand {
                kind: HandKind::Paper
            })
        );
    }

    #[test]
    fn c_is_scissors() {
        let result = FirstHand::from_str("C");
        assert_eq!(
            result,
            Ok(FirstHand {
                kind: HandKind::Scissors
            })
        );
    }

    #[test]
    fn x_is_rock() {
        let result = FirstHand::from_str("X");
        assert_eq!(
            result,
            Ok(FirstHand {
                kind: HandKind::Rock
            })
        );
    }

    #[test]
    fn y_is_paper() {
        let result = FirstHand::from_str("Y");
        assert_eq!(
            result,
            Ok(FirstHand {
                kind: HandKind::Paper
            })
        );
    }

    #[test]
    fn z_is_scissors() {
        let result = FirstHand::from_str("Z");
        assert_eq!(
            result,
            Ok(FirstHand {
                kind: HandKind::Scissors
            })
        );
    }

    #[test]
    fn errors_on_unknown_str() {
        let result = FirstHand::from_str("U");
        assert_eq!(result, Err(ParseError));
    }

    #[test]
    fn errors_on_empty_str() {
        let result = FirstHand::from_str("");
        assert_eq!(result, Err(ParseError));
    }
}
