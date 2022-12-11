use crate::step::Direction;
use crate::step::Step;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::str::Split;

#[derive(Debug)]
pub struct RopeBridge {
    steps: Vec<Step>,
}

impl RopeBridge {
    pub fn new() -> RopeBridge {
        RopeBridge { steps: Vec::new() }
    }

    pub fn add_steps(&mut self, input: &str) {
        let lines: Split<char> = input.split('\n');

        for line in lines {
            if line.is_empty() {
                continue;
            }

            let step: Step = Step::new(line);
            self.steps.push(step);
        }
    }

    pub fn unique_tail_locations(&self, knot_count: i32) -> i32 {
        let mut unique_locations: HashSet<(i32, i32)> = HashSet::new();
        let mut coords: Vec<(i32, i32)> = Vec::new();

        for _ in 0..(knot_count + 1) {
            coords.push((0, 0));
        }

        for step in &self.steps {
            for _ in 0..step.count {
                move_head(&mut coords, &step.direction);

                for index in 1..(coords.len()) {
                    maybe_move_tail(&mut coords, index);
                }

                let (tail_x, tail_y) = coords[coords.len() - 1];
                unique_locations.insert((tail_x, tail_y));
            }
        }

        unique_locations.len() as i32
    }
}

fn move_head(coords: &mut [(i32, i32)], direction: &Direction) {
    let (x, y) = coords[0];
    match direction {
        Direction::Up => coords[0] = (x, y + 1),
        Direction::Down => coords[0] = (x, y - 1),
        Direction::Left => coords[0] = (x - 1, y),
        Direction::Right => coords[0] = (x + 1, y),
    }
}

fn maybe_move_tail(coords: &mut [(i32, i32)], index: usize) {
    let (head_x, head_y) = coords[index - 1];
    let (mut tail_x, mut tail_y) = coords[index];

    if tail_trailing(&head_x, &head_y, &tail_x, &tail_y) {
        move_tail(&head_x, &head_y, &mut tail_x, &mut tail_y);
        coords[index] = (tail_x, tail_y);
    }
}

fn tail_trailing(head_x: &i32, head_y: &i32, tail_x: &i32, tail_y: &i32) -> bool {
    (tail_x - head_x).abs() > 1 || (tail_y - head_y).abs() > 1
}

fn move_tail(head_x: &i32, head_y: &i32, tail_x: &mut i32, tail_y: &mut i32) {
    if head_x == tail_x {
        move_coords(head_y, tail_y);
    } else if head_y == tail_y {
        move_coords(head_x, tail_x);
    } else {
        move_coords(head_y, tail_y);
        move_coords(head_x, tail_x);
    }
}

fn move_coords(head: &i32, tail: &mut i32) {
    match head.cmp(tail) {
        Ordering::Less => *tail -= 1,
        Ordering::Greater => *tail += 1,
        Ordering::Equal => {}
    }
}
