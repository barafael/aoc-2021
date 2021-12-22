use super::Packet;
use crate::day16::{Operator, Type};

impl Packet {
    pub fn eval(&self) -> u64 {
        let op = match self.packet_type {
            Type::Literal(l) => return l,
            Type::Operator(op) => op,
        };
        match op {
            Operator::Sum => self.payload.iter().map(Self::eval).sum::<u64>(),
            Operator::Product => self.payload.iter().map(Self::eval).product::<u64>(),
            Operator::Minimum => self.payload.iter().map(Self::eval).min().unwrap(),
            Operator::Maximum => self.payload.iter().map(Self::eval).max().unwrap(),
            Operator::GreaterThan => {
                if self.payload[0].eval() > self.payload[1].eval() {
                    1
                } else {
                    0
                }
            }
            Operator::LessThan => {
                if self.payload[0].eval() < self.payload[1].eval() {
                    1
                } else {
                    0
                }
            }
            Operator::EqualTo => {
                if self.payload[0].eval() == self.payload[1].eval() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day16::{parse_to_binary, parser::try_parse_packet, test::INPUT};

    pub const THREE: &str = "C200B40A82";
    pub const FIFTYFOUR: &str = "04005AC33890";
    pub const SEVEN: &str = "880086C3E88112";
    pub const NINE: &str = "CE00C43D881120";
    pub const FIVE_IS_LESS: &str = "D8005AC2A8F0";
    pub const FIFTEEN_IS_GREATER: &str = "F600BC2D8F";
    pub const ZERO: &str = "9C005AC2F8F0";
    pub const ONE: &str = "9C0141080250320F1802104A08";

    #[test]
    fn sums_one_two_to_three() {
        let bin = parse_to_binary(THREE);
        let (packet, _) = try_parse_packet(&bin).unwrap();
        assert_eq!(3, packet.eval());
    }

    #[test]
    fn multiplies_six_nine_to_fifty_four() {
        let bin = parse_to_binary(FIFTYFOUR);
        let (packet, _) = try_parse_packet(&bin).unwrap();
        assert_eq!(54, packet.eval());
    }

    #[test]
    fn finds_minimum_is_seven() {
        let bin = parse_to_binary(SEVEN);
        let (packet, _) = try_parse_packet(&bin).unwrap();
        assert_eq!(7, packet.eval());
    }

    #[test]
    fn finds_maximum_is_nine() {
        let bin = parse_to_binary(NINE);
        let (packet, _) = try_parse_packet(&bin).unwrap();
        assert_eq!(9, packet.eval());
    }

    #[test]
    fn five_is_less_than_fifteen() {
        let bin = parse_to_binary(FIVE_IS_LESS);
        let (packet, _) = try_parse_packet(&bin).unwrap();
        assert_eq!(1, packet.eval());
    }

    #[test]
    fn five_is_not_greater_than_fifteen() {
        let bin = parse_to_binary(FIFTEEN_IS_GREATER);
        let (packet, _) = try_parse_packet(&bin).unwrap();
        assert_eq!(0, packet.eval());
    }

    #[test]
    fn five_is_not_fifteen() {
        let bin = parse_to_binary(ZERO);
        let (packet, _) = try_parse_packet(&bin).unwrap();
        assert_eq!(0, packet.eval());
    }

    #[test]
    fn terms_are_equal() {
        let bin = parse_to_binary(ONE);
        let (packet, _) = try_parse_packet(&bin).unwrap();
        assert_eq!(1, packet.eval());
    }

    #[test]
    fn computes_solution() {
        let bin = parse_to_binary(INPUT.trim());
        let (packet, _) = try_parse_packet(&bin).unwrap();
        assert_eq!(167737115857, packet.eval());
    }
}
