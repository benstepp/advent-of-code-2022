use crate::priority::get_priority;

#[derive(Clone, Copy, Debug)]
pub struct Item {
    pub character: char,
    pub priority: i32,
}

impl Item {
    pub fn new(character: char) -> Item {
        let priority: i32 = get_priority(character);
        Item {
            character,
            priority,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower_case_alphabet_new_without_panic() {
        let alphabet: [char; 26] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];

        for letter in alphabet {
            let item = Item::new(letter);
            assert_eq!(item.character, letter);
        }
    }

    #[test]
    fn upper_case_alphabet_new_without_panic() {
        let alphabet: [char; 26] = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];

        for letter in alphabet {
            let item = Item::new(letter);
            assert_eq!(item.character, letter);
        }
    }
}
