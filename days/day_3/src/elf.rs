use crate::item::Item;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Elf {
    pub items: Vec<Item>,
}

impl Elf {
    pub fn new(input: &str) -> Elf {
        let mut items: Vec<Item> = Vec::new();
        for c in input.chars() {
            items.push(Item::new(c));
        }

        Elf { items }
    }

    pub fn item_hash(&self) -> HashSet<Item> {
        let mut hash: HashSet<Item> = HashSet::new();

        for i in &self.items {
            hash.insert(*i);
        }

        hash
    }
}
