#[derive(Debug)]
pub struct Instruction {
    pub number: i32,
    pub from: i32,
    pub to: i32,
}

impl Instruction {
    pub fn new(input: &str) -> Instruction {
        let mut digits: Vec<Vec<char>> = vec![Vec::new(), Vec::new(), Vec::new()];

        let mut mode: i32 = 0;

        for letter in input.chars() {
            if is_digit(letter) {
                digits[mode as usize].push(letter);
            }

            if letter == 'f' || letter == 't' {
                mode += 1;
            }
        }

        let number: i32 = digits[0].iter().collect::<String>().parse::<i32>().unwrap();
        let from: i32 = digits[1].iter().collect::<String>().parse::<i32>().unwrap() - 1;
        let to: i32 = digits[2].iter().collect::<String>().parse::<i32>().unwrap() - 1;

        Instruction { number, from, to }
    }
}

fn is_digit(c: char) -> bool {
    matches!(c, '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0')
}
