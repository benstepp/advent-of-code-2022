use crate::elf::Elf;
use std::str::Split;

pub struct Pair {
    elves: Vec<Elf>,
}

impl Pair {
    pub fn new(input: &str) -> Pair {
        let elf_inputs: Split<char> = input.split(',');
        let mut elves: Vec<Elf> = Vec::new();

        for elf_input in elf_inputs {
            if !elf_input.is_empty() {
                elves.push(Elf::new(elf_input));
            }
        }

        Pair { elves }
    }

    pub fn overlaps(&self) -> bool {
        let a: &Elf = &self.elves[0];
        let b: &Elf = &self.elves[1];

        (a.section_start >= b.section_start && a.section_end <= b.section_end)
            || (b.section_start >= a.section_start && b.section_end <= a.section_end)
    }
}
