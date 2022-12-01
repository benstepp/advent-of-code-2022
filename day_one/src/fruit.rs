#[derive(Debug)]
pub struct Fruit {
    pub calories: i32,
}

impl Fruit {
    pub fn new(input: &str) -> Fruit {
        Fruit {
            calories: input.parse::<i32>().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn panic_if_empty_string_passed() {
        Fruit::new("");
    }

    #[test]
    #[should_panic]
    fn panic_if_invalid_integer_passed() {
        Fruit::new("12a");
    }

    #[test]
    #[should_panic]
    fn panic_if_integer_over_i32_passed() {
        Fruit::new("2147483648");
    }
}
