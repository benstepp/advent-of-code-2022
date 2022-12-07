mod stream;

use crate::stream::Stream;

struct Result {
    start_of_packet: i32,
}

fn main() {
    let input: &str = include_str!("../fixtures/input");
    let result: Result = day_six(input);
    println!("Start of Packet: {}", result.start_of_packet);
}

fn day_six(input: &str) -> Result {
    let start_of_packet: i32 = find_start_of_packet(input);

    Result { start_of_packet }
}

fn find_start_of_packet(input: &str) -> i32 {
    let stream: Stream = Stream::new(input);
    stream.start_packet()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1_returns_7() {
        let input: &str = include_str!("../fixtures/test_input_1");
        let result: Result = day_six(input);
        assert_eq!(result.start_of_packet, 7);
    }

    #[test]
    fn test_input_2_returns_5() {
        let input: &str = include_str!("../fixtures/test_input_2");
        let result: Result = day_six(input);
        assert_eq!(result.start_of_packet, 5);
    }

    #[test]
    fn test_input_3_returns_6() {
        let input: &str = include_str!("../fixtures/test_input_3");
        let result: Result = day_six(input);
        assert_eq!(result.start_of_packet, 6);
    }

    #[test]
    fn test_input_4_returns_10() {
        let input: &str = include_str!("../fixtures/test_input_4");
        let result: Result = day_six(input);
        assert_eq!(result.start_of_packet, 10);
    }

    #[test]
    fn test_input_5_returns_11() {
        let input: &str = include_str!("../fixtures/test_input_5");
        let result: Result = day_six(input);
        assert_eq!(result.start_of_packet, 11);
    }
}
