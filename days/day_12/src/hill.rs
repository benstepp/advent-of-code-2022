use crate::char::get_value;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct Hill {
    matrix: Vec<Vec<i32>>,
    pub start: (usize, usize),
    end: (usize, usize),
}

impl Hill {
    pub fn new(input: &str) -> Hill {
        let mut hill: Hill = Hill::default();

        for (row_index, input_line) in input.split('\n').enumerate() {
            if input_line.is_empty() {
                continue;
            }
            let mut line: Vec<i32> = Vec::new();
            for (col_index, c) in input_line.chars().enumerate() {
                let val = if c == 'S' {
                    hill.start = (row_index, col_index);
                    get_value('a')
                } else if c == 'E' {
                    hill.end = (row_index, col_index);
                    get_value('z')
                } else {
                    get_value(c)
                };

                line.push(val);
            }
            hill.matrix.push(line);
        }

        hill
    }

    pub fn find_scenic_start(&self) -> i32 {
        let mut shortest: i32 = 0;
        for (row_index, row) in self.matrix.iter().enumerate() {
            for (col_index, val) in row.iter().enumerate() {
                if *val == 0 {
                    if let Some(distance_to_end) = self.shortest_path_to_end((row_index, col_index))
                    {
                        if shortest == 0 || distance_to_end < shortest {
                            shortest = distance_to_end;
                        }
                    }
                }
            }
        }

        shortest
    }

    pub fn shortest_path_to_end(&self, start: (usize, usize)) -> Option<i32> {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut depth: i32 = 0;
        queue.push_back(start);

        while !queue.is_empty() {
            let to_visit: Vec<(usize, usize)> = queue.drain(..).collect();

            for coords in to_visit {
                let (row, col) = coords;
                if row == self.end.0 && col == self.end.1 {
                    return Some(depth);
                }

                for child in get_children(&self.matrix, coords) {
                    if !visited.contains(&child) {
                        visited.insert(child);
                        queue.push_back(child);
                    }
                }
            }

            depth += 1;
        }

        None
    }
}

fn get_children(matrix: &[Vec<i32>], coords: (usize, usize)) -> Vec<(usize, usize)> {
    let (row, col) = coords;
    let mut children: Vec<(usize, usize)> = Vec::new();

    let up: i32 = row as i32 - 1;
    if up >= 0 {
        let coord = (up as usize, col);
        if can_traverse(matrix, coords, coord) {
            children.push(coord);
        }
    }

    let down: usize = row + 1;
    if down < matrix.len() {
        let coord = (down, col);
        if can_traverse(matrix, coords, coord) {
            children.push(coord);
        }
    }

    let right: usize = col + 1;
    if right < matrix.get(row).unwrap().len() {
        let coord = (row, right);
        if can_traverse(matrix, coords, coord) {
            children.push(coord);
        }
    }

    let left: i32 = col as i32 - 1;
    if left >= 0 {
        let coord = (row, left as usize);
        if can_traverse(matrix, coords, coord) {
            children.push(coord);
        }
    }

    children
}

fn can_traverse(
    matrix: &[Vec<i32>],
    previous_coord: (usize, usize),
    next_coord: (usize, usize),
) -> bool {
    let previous_value: i32 = get_coord(matrix, previous_coord);
    let next_value: i32 = get_coord(matrix, next_coord);
    previous_value == next_value || previous_value + 1 == next_value || previous_value >= next_value
}

fn get_coord(matrix: &[Vec<i32>], coords: (usize, usize)) -> i32 {
    let (row, col) = coords;
    *matrix.get(row).unwrap().get(col).unwrap()
}
