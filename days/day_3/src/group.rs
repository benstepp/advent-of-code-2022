use crate::elf::Elf;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Group {
    elves: Vec<Elf>,
}

impl Group {
    pub fn new(input: Vec<&str>) -> Group {
        let mut elves: Vec<Elf> = Vec::new();

        for line in input {
            let elf = Elf::new(line);
            elves.push(elf);
        }

        Group { elves }
    }

    pub fn badge_priority(&self) -> i32 {
        let hash_a = self.elves[0].item_hash();
        let hash_b = self.elves[1].item_hash();
        let hash_c = self.elves[2].item_hash();

        let intersection_b: HashSet<_> = hash_a.intersection(&hash_b).cloned().collect();
        let intersection_c: HashSet<_> = intersection_b.intersection(&hash_c).cloned().collect();

        if let Some(item) = intersection_c.iter().next() {
            return item.priority;
        }

        0
    }
}
