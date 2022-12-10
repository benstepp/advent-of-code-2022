pub fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let lines: Vec<&str> = input.split('\n').collect::<Vec<&str>>();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let mut row: Vec<i32> = Vec::new();
        for number in line.chars() {
            let num: i32 = number.to_string().parse::<i32>().unwrap();
            row.push(num);
        }

        matrix.push(row);
    }

    matrix
}
