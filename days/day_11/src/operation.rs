#[derive(Debug, Clone)]
pub enum Input {
    Number(i64),
    Old,
}

#[derive(Debug, Clone)]
pub enum Operator {
    Multiplication,
    Division,
    Addition,
    Subtraction,
}

#[derive(Debug, Clone)]
pub struct Operation {
    left: Input,
    right: Input,
    operator: Operator,
}

impl Operation {
    pub fn new(input: &str) -> Operation {
        let operator: Operator = get_operator(input);
        let parts = input.split(get_splitter(&operator)).collect::<Vec<&str>>();

        Operation {
            operator,
            left: parse_input(parts[0]),
            right: parse_input(parts[1]),
        }
    }

    pub fn perform(&self, number: i64) -> i64 {
        match self.operator {
            Operator::Multiplication => self.perform_multiplication(number),
            Operator::Subtraction => self.perform_subtraction(number),
            Operator::Addition => self.perform_addition(number),
            Operator::Division => self.perform_division(number),
        }
    }

    fn perform_multiplication(&self, number: i64) -> i64 {
        self.get_left(number) * self.get_right(number)
    }

    fn perform_subtraction(&self, number: i64) -> i64 {
        self.get_left(number) - self.get_right(number)
    }

    fn perform_addition(&self, number: i64) -> i64 {
        self.get_left(number) + self.get_right(number)
    }

    fn perform_division(&self, number: i64) -> i64 {
        self.get_left(number) / self.get_right(number)
    }

    fn get_left(&self, number: i64) -> i64 {
        match self.left {
            Input::Number(val) => val,
            Input::Old => number,
        }
    }

    fn get_right(&self, number: i64) -> i64 {
        match self.right {
            Input::Number(val) => val,
            Input::Old => number,
        }
    }
}

impl Default for Operation {
    fn default() -> Self {
        Self {
            left: Input::Old,
            right: Input::Old,
            operator: Operator::Addition,
        }
    }
}

fn get_operator(input: &str) -> Operator {
    if input.contains('*') {
        Operator::Multiplication
    } else if input.contains('+') {
        Operator::Addition
    } else if input.contains('-') {
        Operator::Subtraction
    } else if input.contains('/') {
        Operator::Division
    } else {
        panic!("invalid operation {}", input);
    }
}

const MULTIPLICATION: &str = " * ";
const SUBTRACTION: &str = " - ";
const ADDITION: &str = " + ";
const DIVISION: &str = " / ";

fn get_splitter(operator: &Operator) -> &'static str {
    match operator {
        Operator::Multiplication => MULTIPLICATION,
        Operator::Subtraction => SUBTRACTION,
        Operator::Addition => ADDITION,
        Operator::Division => DIVISION,
    }
}

fn parse_input(input: &str) -> Input {
    if input == "old" {
        Input::Old
    } else {
        let num: i64 = input.parse::<i64>().unwrap();
        Input::Number(num)
    }
}
