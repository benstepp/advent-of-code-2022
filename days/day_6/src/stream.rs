pub struct Stream {
    letters: Vec<char>,
}

impl Stream {
    pub fn new(input: &str) -> Stream {
        let letters: Vec<char> = input.chars().collect();
        Stream { letters }
    }

    pub fn start_packet(&self) -> i32 {
        let mut stack: Vec<char> = Vec::new();

        for (index, letter) in self.letters.iter().enumerate() {
            if is_start_packet(&stack) {
                return index as i32;
            }

            if stack.len() == 4 {
                stack.remove(0);
            }

            stack.push(*letter);
        }

        0
    }
}

fn is_start_packet(packet: &Vec<char>) -> bool {
    packet.len() == 4 && is_unique(packet)
}

fn is_unique(packet: &[char]) -> bool {
    packet[0] != packet[1]
        && packet[0] != packet[2]
        && packet[0] != packet[3]
        && packet[1] != packet[2]
        && packet[1] != packet[3]
        && packet[2] != packet[3]
}
