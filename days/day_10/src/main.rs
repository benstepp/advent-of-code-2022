mod device;
mod instruction;

use crate::device::Device;

struct Result {
    one: i32,
    two: Vec<char>,
}

fn main() {
    let input = include_str!("../fixtures/input");
    let result = day_ten(input);
    println!("Sum: {}", result.one);
    println!("Puzzle Two:");
    format(result.two);
}

fn day_ten(input: &str) -> Result {
    let mut device: Device = Device::new();
    device.add_instructions(input);

    let one: i32 = device.signal_strength(20)
        + device.signal_strength(60)
        + device.signal_strength(100)
        + device.signal_strength(140)
        + device.signal_strength(180)
        + device.signal_strength(220);

    let two: Vec<char> = device.draw();

    Result { one, two }
}

fn format(letters: Vec<char>) {
    for row in 0..6 {
        let slice = &letters[(row * 40)..(40 * (row + 1))];
        println!("{}", slice.iter().collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_works() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_ten(input);
        assert_eq!(result.one, 13140);

        let output: Vec<char> = "##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....".chars().collect::<Vec<char>>();

        for (index, c) in output.iter().enumerate() {
            assert_eq!(c, &result.two[index], "incorrect index {}", index);
        }

        assert_eq!(result.two, output)
    }
}
