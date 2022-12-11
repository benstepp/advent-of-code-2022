#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Step {
    pub direction: Direction,
    pub count: i32,
}

impl Step {
    pub fn new(input: &str) -> Step {
        let parts: Vec<&str> = input.split(' ').collect::<Vec<&str>>();
        let direction: Direction = determine_direction(parts[0]);
        let count: i32 = parts[1].parse::<i32>().unwrap();

        Step { direction, count }
    }
}

fn determine_direction(input: &str) -> Direction {
    match input {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("invalid direction {}", input),
    }
}
