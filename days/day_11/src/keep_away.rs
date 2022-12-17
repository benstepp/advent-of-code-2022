use crate::monkey::Monkey;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub struct KeepAway {
    monkeys: Vec<Monkey>,
    worry_reduction: bool,
    least_common_divisor: i64,
}

impl KeepAway {
    pub fn new(input: &str) -> KeepAway {
        let mut monkeys: Vec<Monkey> = Vec::new();

        for i in input.split("\n\n") {
            let monkey: Monkey = Monkey::new(i);
            monkeys.push(monkey);
        }

        let least_common_divisor: i64 = least_common_divisor(&monkeys);

        KeepAway {
            monkeys,
            worry_reduction: true,
            least_common_divisor,
        }
    }

    pub fn round(&mut self) {
        let mut thrown_items: HashMap<usize, Vec<i64>> = HashMap::new();

        for m in 0..self.monkeys.len() {
            let mut monkey: &mut Monkey = &mut self.monkeys[m];
            let mut items: Vec<i64> = monkey.items.clone();

            if let Some(thrown) = thrown_items.remove(&m) {
                for t in thrown {
                    items.push(t);
                }
            }

            monkey.items = Vec::new();
            monkey.inspections_performed += items.len() as i32;

            for item in items {
                let worry_level: i64 = monkey.compute_worry_level(item);
                let reduced: i64 = worry_level % self.least_common_divisor;

                let value = if self.worry_reduction {
                    reduced / 3
                } else {
                    reduced
                };

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

    pub fn monkey_business(&self) -> i64 {
        let mut max_heap: BinaryHeap<i64> = BinaryHeap::new();

        for monkey in &self.monkeys {
            max_heap.push(monkey.inspections_performed as i64);
        }

        max_heap.pop().unwrap() * max_heap.pop().unwrap()
    }

    pub fn set_worry_reduction(&mut self, value: bool) {
        self.worry_reduction = value
    }
}

fn least_common_divisor(monkeys: &[Monkey]) -> i64 {
    let mut set: HashSet<i64> = HashSet::new();

    for m in monkeys {
        set.insert(m.divisor);
    }

    set.iter().product()
}
