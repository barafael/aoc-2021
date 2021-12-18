use super::Packet;

impl Packet {
    pub fn version_sum(&self) -> usize {
        self.version as usize
            + self
                .payload
                .iter()
                .map(|packet| packet.version_sum())
                .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use crate::day16::{
        parse_to_binary,
        parser::try_parse_packet,
        test::{EXAMPLE_0, EXAMPLE_0_1, EXAMPLE_1, EXAMPLE_2, EXAMPLE_3, EXAMPLE_4, INPUT},
        Operator, Packet, Type,
    };

    #[test]
    fn parses_example_0() {
        let bin = parse_to_binary(EXAMPLE_0);
        let (packet, remainder) = try_parse_packet(&bin).unwrap();
        assert_eq!(
            Packet {
                version: 1,
                packet_type: Type::Operator(Operator::Dummy(6)),
                payload: vec![
                    Packet {
                        version: 6,
                        packet_type: Type::Literal(10),
                        payload: vec![],
                    },
                    Packet {
                        version: 2,
                        packet_type: Type::Literal(20),
                        payload: vec![],
                    },
                ],
            },
            packet
        );
        assert_eq!(&[0, 0, 0, 0, 0, 0, 0], remainder);
    }

    #[test]
    fn parses_example_0_1() {
        let bin = parse_to_binary(EXAMPLE_0_1);
        let (packet, remainder) = try_parse_packet(&bin).unwrap();
        assert_eq!(
            Packet {
                version: 7,
                packet_type: Type::Operator(Operator::Dummy(3)),
                payload: vec![
                    Packet {
                        version: 2,
                        packet_type: Type::Literal(1),
                        payload: vec![],
                    },
                    Packet {
                        version: 4,
                        packet_type: Type::Literal(2),
                        payload: vec![],
                    },
                    Packet {
                        version: 1,
                        packet_type: Type::Literal(3),
                        payload: vec![],
                    },
                ],
            },
            packet
        );
        assert_eq!(&[0, 0, 0, 0, 0], remainder);
    }

    #[test]
    fn parses_example_1() {
        let bin = parse_to_binary(EXAMPLE_1);
        let (packet, _) = try_parse_packet(&bin).unwrap();
        assert_eq!(16, packet.version_sum());
    }

    #[test]
    fn parses_example_2() {
        let bin = parse_to_binary(EXAMPLE_2);
        let (packet, _) = try_parse_packet(&bin).unwrap();
        assert_eq!(12, packet.version_sum());
    }

    #[test]
    fn parses_example_3() {
        let bin = parse_to_binary(EXAMPLE_3);
        let (packet, _) = try_parse_packet(&bin).unwrap();
        assert_eq!(23, packet.version_sum());
    }

    #[test]
    fn parses_example_4() {
        let bin = parse_to_binary(EXAMPLE_4);
        let (packet, _) = try_parse_packet(&bin).unwrap();
        assert_eq!(31, packet.version_sum());
    }

    #[test]
    fn computes_solution() {
        let bin = parse_to_binary(INPUT.trim());
        let (packet, _) = try_parse_packet(&bin).unwrap();
        assert_eq!(943, packet.version_sum());
    }
}
