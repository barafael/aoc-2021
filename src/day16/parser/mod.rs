use self::{literal::try_parse, operator::try_parse_operator};
use super::{Packet, Type};
use num_traits::FromPrimitive;

pub mod literal;
pub mod operator;

pub fn try_parse_version(input: &[u8]) -> Option<(u8, &[u8])> {
    let mut version = 0u8;
    if input.get(0)? == &1 {
        version |= 0b100;
    }
    if input.get(1)? == &1 {
        version |= 0b10;
    }
    if input.get(2)? == &1 {
        version |= 0b1;
    }
    Some((version, &input[3..]))
}

pub fn try_parse_type_id(input: &[u8]) -> Option<(u8, &[u8])> {
    let mut type_id = 0u8;
    if input.get(0)? == &1 {
        type_id |= 0b100;
    }
    if input.get(1)? == &1 {
        type_id |= 0b10;
    }
    if input.get(2)? == &1 {
        type_id |= 0b1;
    }
    Some((type_id, &input[3..]))
}

pub fn try_parse_packet(input: &[u8]) -> Option<(Packet, &[u8])> {
    let (version, remainder) = try_parse_version(input)?;
    let (type_id, remainder) = try_parse_type_id(remainder)?;
    if type_id == 4 {
        let (literal, remainder) = try_parse(remainder)?;
        Some((
            Packet {
                version,
                packet_type: Type::Literal(literal),
                payload: vec![],
            },
            remainder,
        ))
    } else {
        let (packets, remainder) = try_parse_operator(remainder)?;
        Some((
            Packet {
                version,
                packet_type: Type::Operator(FromPrimitive::from_u8(type_id)?),
                payload: packets,
            },
            remainder,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::try_parse_packet;
    use crate::day16::{Operator, Packet, Type};

    #[test]
    fn parses_literal_packet() {
        let input = [
            1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0,
        ];
        let (packet, remainder) = try_parse_packet(&input).unwrap();
        assert_eq!(
            Packet {
                version: 6,
                packet_type: Type::Literal(2021),
                payload: vec![],
            },
            packet
        );
        assert_eq!(&[0, 0, 0], remainder);
    }

    #[test]
    fn parses_mode0_operator_packet() {
        let input = [
            0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0,
            1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let (packet, remainder) = try_parse_packet(&input).unwrap();
        assert_eq!(
            Packet {
                version: 1,
                packet_type: Type::Operator(Operator::LessThan),
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
}
