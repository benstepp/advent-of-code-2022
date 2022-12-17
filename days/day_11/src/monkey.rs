use crate::operation::Operation;

#[derive(Debug, Clone, Default)]
pub struct Monkey {
    pub items: Vec<i32>,
    pub inspections_performed: i32,
    index: usize,
    divisor: i32,
    operation: Operation,
    throw_true: usize,
    throw_false: usize,
}

impl Monkey {
    pub fn new(input: &str) -> Monkey {
        let mut monkey: Monkey = Monkey::default();

        for line in input.split('\n') {
            let l: &str = line.trim();

            if let Some(value) = l.strip_prefix("Starting items: ") {
                monkey.items = make_items(value);
            }

            if let Some(value) = l.strip_prefix("If true: throw to monkey ") {
                monkey.throw_true = value.parse::<usize>().unwrap();
            }

            if let Some(value) = l.strip_prefix("If false: throw to monkey ") {
                monkey.throw_false = value.parse::<usize>().unwrap();
            }

            if let Some(value) = l.strip_prefix("Test: divisible by ") {
                monkey.divisor = value.parse::<i32>().unwrap();
            }

            if let Some(value) = l.strip_prefix("Operation: new = ") {
                monkey.operation = Operation::new(value);
            }

            if let Some(value) = l.strip_prefix("Monkey ") {
                monkey.index = value.strip_suffix(':').unwrap().parse::<usize>().unwrap();
            }
        }

        monkey
    }

    pub fn compute_worry_level(&self, item: i32) -> i32 {
        let new_worry_level: i32 = self.operation.perform(item);
        let rounded: i32 = new_worry_level / 3;
        rounded
    }

    pub fn get_throw_target(&self, item: i32) -> usize {
        if item % self.divisor == 0 {
            self.throw_true
        } else {
            self.throw_false
        }
    }
}

fn make_items(input: &str) -> Vec<i32> {
    let mut items: Vec<i32> = Vec::new();

    for i in input.split(", ") {
        items.push(i.parse::<i32>().unwrap());
    }

    items
}
