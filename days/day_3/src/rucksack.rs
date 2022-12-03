use crate::item::Item;
use std::collections::HashSet;
use std::slice::Chunks;

pub struct Rucksack {
    compartments: Vec<Vec<Item>>,
}

impl Rucksack {
    pub fn new(input: &str) -> Rucksack {
        let half = input.len() / 2;
        let binding = input.chars().collect::<Vec<char>>();
        let chunks: Chunks<char> = binding.chunks(half);
        let mut compartments: Vec<Vec<Item>> = Vec::new();

        for chunk in chunks {
            let mut compartment: Vec<Item> = Vec::new();
            for c in chunk {
                compartment.push(Item::new(*c));
            }
            compartments.push(compartment);
        }

        Rucksack { compartments }
    }

    pub fn priority(&self) -> i32 {
        let mut compartments = self.compartments.clone();
        let mut hash: HashSet<char> = HashSet::new();

        let first = compartments.remove(0);

        for item in &first {
            hash.insert(item.character);
        }

        for compartment in compartments {
            for item in compartment {
                if hash.contains(&item.character) {
                    return item.priority;
                }
            }
        }

        0
    }
}
