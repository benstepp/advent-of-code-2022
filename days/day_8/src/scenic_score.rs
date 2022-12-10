use std::cmp::Ordering;

pub fn max_scenic_score(matrix: Vec<Vec<i32>>) -> i32 {
    let scores = build(matrix);
    let mut max: i32 = scores[0][0];

    for row in scores.iter() {
        for col in row.iter() {
            if *col > max {
                max = *col;
            }
        }
    }

    max
}

fn build(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut scores: Vec<Vec<i32>> = Vec::new();

    for (row, trees) in matrix.iter().enumerate() {
        let mut scores_row = Vec::new();

        for (col, tree) in trees.iter().enumerate() {
            let score = scenic_score(&matrix, row, col, *tree);
            scores_row.push(score);
        }

        scores.push(scores_row)
    }

    scores
}

fn scenic_score(matrix: &[Vec<i32>], row: usize, col: usize, max: i32) -> i32 {
    top_score(matrix, row, col, max)
        * bottom_score(matrix, row, col, max)
        * left_score(matrix, row, col, max)
        * right_score(matrix, row, col, max)
}

fn top_score(matrix: &[Vec<i32>], row: usize, col: usize, max: i32) -> i32 {
    let mut up: i32 = col as i32 - 1;
    let mut count: i32 = 0;

    while up >= 0 {
        match matrix[row][up as usize].cmp(&max) {
            Ordering::Less => {
                count += 1;
                up -= 1;
            }
            Ordering::Equal => {
                count += 1;
                break;
            }
            Ordering::Greater => {
                count += 1;
                break;
            }
        }
    }

    count
}

fn bottom_score(matrix: &[Vec<i32>], row: usize, col: usize, max: i32) -> i32 {
    let mut down: i32 = row as i32 + 1;
    let until = matrix[0].len() as i32;
    let mut count: i32 = 0;

    while down < until {
        match matrix[down as usize][col].cmp(&max) {
            Ordering::Less => {
                count += 1;
                down += 1;
            }
            Ordering::Equal => {
                count += 1;
                break;
            }
            Ordering::Greater => {
                count += 1;
                break;
            }
        }
    }

    count
}

fn left_score(matrix: &[Vec<i32>], row: usize, col: usize, max: i32) -> i32 {
    let mut left: i32 = row as i32 - 1;
    let mut count: i32 = 0;

    while left >= 0 {
        match matrix[left as usize][col].cmp(&max) {
            Ordering::Less => {
                count += 1;
                left -= 1;
            }
            Ordering::Equal => {
                count += 1;
                break;
            }
            Ordering::Greater => {
                count += 1;
                break;
            }
        }
    }

    count
}

fn right_score(matrix: &[Vec<i32>], row: usize, col: usize, max: i32) -> i32 {
    let mut right: i32 = col as i32 + 1;
    let until = matrix.len() as i32;
    let mut count: i32 = 0;

    while right < until {
        match matrix[row][right as usize].cmp(&max) {
            Ordering::Less => {
                count += 1;
                right += 1;
            }
            Ordering::Equal => {
                count += 1;
                break;
            }
            Ordering::Greater => {
                count += 1;
                break;
            }
        }
    }

    count
}
