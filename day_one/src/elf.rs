use crate::fruit::Fruit;
use std::str::Split;

#[derive(Debug)]
pub struct Elf {
    pub fruits: Vec<Fruit>,
    pub calories: i32,
}

impl Elf {
    pub fn new(input: &str) -> Elf {
        let fruit_inputs: Split<char> = input.split('\n');
        let mut fruits: Vec<Fruit> = Vec::new();
        let mut calories: i32 = 0;

        for fruit_input in fruit_inputs {
            if !fruit_input.is_empty() {
                let fruit: Fruit = Fruit::new(fruit_input);
                calories += fruit.calories;
                fruits.push(fruit);
            }
        }

        Elf { fruits, calories }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_fruits_when_no_input() {
        let elf: Elf = Elf::new("");
        assert_eq!(elf.fruits.len(), 0);
    }

    #[test]
    fn zero_calories_when_no_input() {
        let elf: Elf = Elf::new("");
        assert_eq!(elf.calories, 0);
    }

    #[test]
    fn removes_leading_empty_line() {
        let input: String = ["", "1000", "2000"].join("\n");

        let elf: Elf = Elf::new(&input);
        assert_eq!(elf.fruits.len(), 2)
    }

    #[test]
    fn removes_leading_empty_lines() {
        let input: String = ["", "", "1000", "2000"].join("\n");

        let elf: Elf = Elf::new(&input);
        assert_eq!(elf.fruits.len(), 2)
    }

    #[test]
    fn removes_trailing_empty_line() {
        let input: String = ["1000", "2000", ""].join("\n");

        let elf: Elf = Elf::new(&input);
        assert_eq!(elf.fruits.len(), 2)
    }

    #[test]
    fn removes_trailing_empty_lines() {
        let input: String = ["1000", "2000", "", ""].join("\n");

        let elf: Elf = Elf::new(&input);
        assert_eq!(elf.fruits.len(), 2)
    }

    #[test]
    #[should_panic]
    fn panic_if_invalid_integer_inputs() {
        let input: String = ["abc"].join("\n");

        Elf::new(&input);
    }

    #[test]
    fn sums_up_calories_for_an_elf() {
        let input: String = ["1000", "2000", "7040"].join("\n");

        let elf: Elf = Elf::new(&input);
        assert_eq!(elf.calories, 10040)
    }
}
