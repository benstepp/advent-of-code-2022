mod stream;

use crate::stream::Stream;

struct Result {
    start_of_packet_4: i32,
    start_of_packet_14: i32,
}

fn main() {
    let input: &str = include_str!("../fixtures/input");
    let result: Result = day_six(input);
    println!("Start of Packet (4): {}", result.start_of_packet_4);
    println!("Start of Packet (14): {}", result.start_of_packet_14);
}

fn day_six(input: &str) -> Result {
    let stream: Stream = Stream::new(input);
    let start_of_packet_4: i32 = stream.start_packet(4);
    let start_of_packet_14: i32 = stream.start_packet(14);

    Result {
        start_of_packet_4,
        start_of_packet_14,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1_returns_7() {
        let input: &str = include_str!("../fixtures/test_input_1");
        let result: Result = day_six(input);
        assert_eq!(result.start_of_packet_4, 7);
        assert_eq!(result.start_of_packet_14, 19);
    }

    #[test]
    fn test_input_2_returns_5() {
        let input: &str = include_str!("../fixtures/test_input_2");
        let result: Result = day_six(input);
        assert_eq!(result.start_of_packet_4, 5);
        assert_eq!(result.start_of_packet_14, 23);
    }

    #[test]
    fn test_input_3_returns_6() {
        let input: &str = include_str!("../fixtures/test_input_3");
        let result: Result = day_six(input);
        assert_eq!(result.start_of_packet_4, 6);
        assert_eq!(result.start_of_packet_14, 23);
    }

    #[test]
    fn test_input_4_returns_10() {
        let input: &str = include_str!("../fixtures/test_input_4");
        let result: Result = day_six(input);
        assert_eq!(result.start_of_packet_4, 10);
        assert_eq!(result.start_of_packet_14, 29);
    }

    #[test]
    fn test_input_5_returns_11() {
        let input: &str = include_str!("../fixtures/test_input_5");
        let result: Result = day_six(input);
        assert_eq!(result.start_of_packet_4, 11);
        assert_eq!(result.start_of_packet_14, 26);
    }
}
