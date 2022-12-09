mod file;
mod filesystem;

use crate::filesystem::Filesystem;

struct Result {
    under_100000_sum: i32,
}

fn main() {
    let input: &str = include_str!("../fixtures/input");
    let result: Result = day_seven(input);
    println!("Under 100000: {}", result.under_100000_sum);
}

fn day_seven(input: &str) -> Result {
    let fs: Filesystem = Filesystem::new(input);
    let under_100000_sum: i32 = fs.sum_of_directories_under(100000);
    Result { under_100000_sum }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_works() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_seven(input);
        assert_eq!(result.under_100000_sum, 95437);
    }
}
