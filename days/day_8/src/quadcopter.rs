pub fn find_visible_trees(matrix: Vec<Vec<i32>>) -> i32 {
    let mut visible_trees: i32 = 0;

    for (row, trees) in matrix.iter().enumerate() {
        for (col, tree) in trees.iter().enumerate() {
            if visible(&matrix, row, col, *tree) {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

fn visible(matrix: &[Vec<i32>], row: usize, col: usize, max: i32) -> bool {
    visible_from_top(matrix, row, col, max)
        || visible_from_bottom(matrix, row, col, max)
        || visible_from_left(matrix, row, col, max)
        || visible_from_right(matrix, row, col, max)
}

fn visible_from_top(matrix: &[Vec<i32>], row: usize, col: usize, max: i32) -> bool {
    let mut up: i32 = col as i32 - 1;

    while up >= 0 {
        if matrix[row][up as usize] >= max {
            return false;
        } else {
            up -= 1;
        }
    }

    true
}

fn visible_from_bottom(matrix: &[Vec<i32>], row: usize, col: usize, max: i32) -> bool {
    let mut down: i32 = row as i32 + 1;
    let until = matrix[0].len() as i32;

    while down < until {
        if matrix[down as usize][col] >= max {
            return false;
        } else {
            down += 1;
        }
    }

    true
}

fn visible_from_left(matrix: &[Vec<i32>], row: usize, col: usize, max: i32) -> bool {
    let mut left: i32 = row as i32 - 1;

    while left >= 0 {
        if matrix[left as usize][col] >= max {
            return false;
        } else {
            left -= 1;
        }
    }

    true
}

fn visible_from_right(matrix: &[Vec<i32>], row: usize, col: usize, max: i32) -> bool {
    let mut right: i32 = col as i32 + 1;
    let until = matrix.len() as i32;

    while right < until {
        if matrix[row][right as usize] >= max {
            return false;
        } else {
            right += 1;
        }
    }

    true
}
