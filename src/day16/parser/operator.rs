use super::try_parse_packet;
use crate::day16::Packet;

pub fn try_parse_operator(input: &[u8]) -> Option<(Vec<Packet>, &[u8])> {
    let mode = input[0];
    Some(if mode == 0 {
        try_parse_mode0_operator_packets(&input[1..])?
    } else if mode == 1 {
        try_parse_mode1_operator_packets(&input[1..])?
    } else {
        return None;
    })
}

fn try_parse_mode0_operator_packets(input: &[u8]) -> Option<(Vec<Packet>, &[u8])> {
    let num_bits_slice = &input[..15];
    let mut num_bits: usize = 0;
    for (index, bit) in num_bits_slice.iter().rev().enumerate() {
        if *bit != 0 {
            num_bits |= 1 << index;
        }
    }
    let mut remainder = &input[15..];
    let mut remaining_bits = num_bits;
    let mut packets = vec![];
    loop {
        let (packet, r) = try_parse_packet(remainder)?;
        packets.push(packet);
        remaining_bits -= remainder.len() - r.len();
        remainder = r;
        if remaining_bits == 0 {
            break;
        }
    }

    Some((packets, remainder))
}

fn try_parse_mode1_operator_packets(input: &[u8]) -> Option<(Vec<Packet>, &[u8])> {
    let num_packets_slice = &input[..11];
    let mut num_packets: usize = 0;
    for (index, bit) in num_packets_slice.iter().rev().enumerate() {
        if *bit != 0 {
            num_packets |= 1 << index;
        }
    }
    let mut remainder = &input[11..];
    let mut packets = vec![];
    for _i in 0..num_packets {
        let (packet, r) = try_parse_packet(remainder)?;
        packets.push(packet);
        remainder = r;
    }

    Some((packets, remainder))
}

#[cfg(test)]
mod tests {
    use crate::day16::{
        parser::operator::{try_parse_mode0_operator_packets, try_parse_mode1_operator_packets},
        Packet, Type,
    };

    #[test]
    fn parses_mode0_operator_packet() {
        let input = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0,
            1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let (packet, remainder) = try_parse_mode0_operator_packets(&input).unwrap();
        assert_eq!(
            vec![
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
            packet
        );
        assert_eq!(&[0, 0, 0, 0, 0, 0, 0], remainder);
    }

    #[test]
    fn parses_mode1_operator_packet() {
        let input = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
        ];
        let (packet, remainder) = try_parse_mode1_operator_packets(&input).unwrap();
        assert_eq!(
            vec![
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
            packet
        );
        assert_eq!(&[0, 0, 0, 0, 0], remainder);
    }
}
