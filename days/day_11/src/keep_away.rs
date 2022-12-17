use crate::monkey::Monkey;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug)]
pub struct KeepAway {
    monkeys: Vec<Monkey>,
}

impl KeepAway {
    pub fn new(input: &str) -> KeepAway {
        let mut monkeys: Vec<Monkey> = Vec::new();

        for i in input.split("\n\n") {
            let monkey: Monkey = Monkey::new(i);
            monkeys.push(monkey);
        }

        KeepAway { monkeys }
    }

    pub fn round(&mut self) {
        let mut thrown_items: HashMap<usize, Vec<i32>> = HashMap::new();

        for m in 0..self.monkeys.len() {
            let mut monkey: &mut Monkey = &mut self.monkeys[m];
            let mut items: Vec<i32> = monkey.items.clone();

            if let Some(thrown) = thrown_items.remove(&m) {
                for t in thrown {
                    items.push(t);
                }
            }

            monkey.items = Vec::new();
            monkey.inspections_performed += items.len() as i32;

            for item in items {
                let value: i32 = monkey.compute_worry_level(item);

                let target: usize = monkey.get_throw_target(value);

                thrown_items
                    .entry(target)
                    .and_modify(|items| items.push(value))
                    .or_insert_with(|| vec![value]);
            }
        }

        for (m, items) in thrown_items.drain() {
            for i in items {
                self.monkeys[m].items.push(i);
            }
        }
    }

    pub fn rounds(&mut self, rounds: i32) {
        for _ in 0..rounds {
            self.round();
        }
    }

    pub fn monkey_business(&self) -> i32 {
        let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();

        for monkey in &self.monkeys {
            max_heap.push(monkey.inspections_performed);
        }

        max_heap.pop().unwrap() * max_heap.pop().unwrap()
    }
}
