use num_derive::{FromPrimitive, ToPrimitive};

pub mod parser;
pub mod problem_1;
pub mod problem_2;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Literal(u64),
    Operator(Operator),
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive, ToPrimitive, Eq)]
#[non_exhaustive]
pub enum Operator {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Packet {
    version: u8,
    packet_type: Type,
    payload: Vec<Packet>,
}

fn num_to_arr4(num: u8) -> [u8; 4] {
    [
        (num & 0b1000 != 0) as u8,
        (num & 0b0100 != 0) as u8,
        (num & 0b0010 != 0) as u8,
        (num & 0b0001 != 0) as u8,
    ]
}

pub fn parse_to_binary(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .flat_map(num_to_arr4)
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day16::parse_to_binary;

    pub const INPUT: &str = include_str!("../../input/day16.txt");

    pub const EXAMPLE_0: &str = "38006F45291200";
    pub const EXAMPLE_0_1: &str = "EE00D40C823060";
    pub const EXAMPLE_1: &str = "8A004A801A8002F478";
    pub const EXAMPLE_2: &str = "620080001611562C8802118E34";
    pub const EXAMPLE_3: &str = "C0015000016115A2E0802F182340";
    pub const EXAMPLE_4: &str = "A0016C880162017C3686B18A3D4780";

    #[test]
    fn parses_simple_message() {
        let result = parse_to_binary("ABCDE1234");
        assert_eq!(
            vec![
                1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0,
                0, 0, 1, 1, 0, 1, 0, 0,
            ],
            result
        );
    }
}
