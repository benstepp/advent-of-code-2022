mod device;
mod instruction;

use crate::device::Device;

struct Result {
    one: i32,
}

fn main() {
    let input = include_str!("../fixtures/input");
    let result = day_ten(input);
    println!("Sum: {}", result.one);
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

    Result { one }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_works() {
        let input: &str = include_str!("../fixtures/test_input");
        let result: Result = day_ten(input);
        assert_eq!(result.one, 13140);
    }
}
