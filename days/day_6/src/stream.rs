use std::collections::HashSet;

pub struct Stream {
    letters: Vec<char>,
}

impl Stream {
    pub fn new(input: &str) -> Stream {
        let letters: Vec<char> = input.chars().collect();
        Stream { letters }
    }

    pub fn start_packet(&self, len: usize) -> i32 {
        let mut stack: Vec<char> = Vec::new();

        for (index, letter) in self.letters.iter().enumerate() {
            if stack.len() == len && is_unique(&stack) {
                return index as i32;
            }

            if stack.len() == len {
                stack.remove(0);
            }

            stack.push(*letter);
        }

        self.letters.len() as i32
    }
}

fn is_unique(packet: &[char]) -> bool {
    let mut set: HashSet<char> = HashSet::new();

    for letter in packet {
        set.insert(*letter);
    }

    set.len() == packet.len()
}
