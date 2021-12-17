use nom::bits::complete::take;
use nom::bytes::complete::take_while;
use nom::combinator::not;
use nom::{bytes::complete::take_while_m_n, combinator::map_res, IResult};

use super::{Literal, Packet};

//fn parse_version(input: (&[u8], usize), count: usize) -> IResult<(&[u8], usize), u8> {}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn version(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(3usize)(input)
}

fn type_id(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(3usize)(input)
}

fn literal_group(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(5usize)(input)
}

fn is_last_group(group: &u8) -> bool {
    group & 0b10000 != 0
}

fn parse_literal(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet<Literal>> {
    let (input, version) = version(input)?;
    let (input, type_id) = nom::bits::complete::tag(0b100u8, 3usize)(input)?;
    println!("{:b} {:b}", input.0[0], input.0[1]);
    println!("{}", input.1);
    let mut literal = 0u64;
    let mut input = input;

    let x = take_while(is_last_group)(input);

    let literal = Literal(literal);
    Ok((
        input,
        Packet {
            version,
            type_id,
            payload: literal,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::{parse_literal, type_id, version};
    use crate::day16::{parser::literal_group, Literal, Packet};
    use nom::{
        bits::complete::take,
        error::{Error, ErrorKind},
        IResult,
    };

    #[test]
    fn parses_version() {
        let first_byte = &[0b10100010];
        let ((bits, offset), version) = version((first_byte.as_ref(), 0)).unwrap();
        assert_eq!(0b00000101, version);
        assert_eq!(0b10100010, bits[0]);
        assert_eq!(3, offset);
    }

    #[test]
    fn parses_type() {
        let first_byte = &[0b10010010];
        let ((bits, offset), type_id) = type_id((first_byte.as_ref(), 3)).unwrap();
        assert_eq!(0b00000100, type_id);
        assert_eq!(0b10010010, bits[0]);
        assert_eq!(6, offset);
    }

    #[test]
    fn parses_literal_group() {
        let bytes = &[0b10010010, 0b11010101];
        let ((remainder, offset), group) = literal_group((bytes.as_ref(), 6)).unwrap();
        assert_eq!(0b0010110, group);
        assert_eq!([0b11010101].as_ref(), remainder);
        assert_eq!(3, offset);
    }

    #[test]
    fn parses_literal_packet() {
        let data = [0b10110010, 0b00101011];
        let ((rem, count), packet) = parse_literal((data.as_ref(), 0)).unwrap();
        assert_eq!(
            Packet {
                version: 0b00000101,
                type_id: 0b00000100,
                payload: Literal(0b11011),
            },
            packet
        );
    }

    #[test]
    fn parses_bits() {
        // Input is a tuple of (input: I, bit_offset: usize)
        fn parser(input: (&[u8], usize), count: usize) -> IResult<(&[u8], usize), u8> {
            take(count)(input)
        }

        // Consumes 0 bits, returns 0
        assert_eq!(
            parser(([0b00010010].as_ref(), 0), 0),
            Ok((([0b00010010].as_ref(), 0), 0))
        );

        // Consumes 4 bits, returns their values and increase offset to 4
        assert_eq!(
            parser(([0b00010010].as_ref(), 0), 4),
            Ok((([0b00010010].as_ref(), 4), 0b00000001))
        );

        // Consumes 4 bits, offset is 4, returns their values and increase offset to 0 of next byte
        assert_eq!(
            parser(([0b00010010].as_ref(), 4), 4),
            Ok((([].as_ref(), 0), 0b00000010))
        );

        // Tries to consume 12 bits but only 8 are available
        assert_eq!(
            parser(([0b00010010].as_ref(), 0), 12),
            Err(nom::Err::Error(Error {
                input: ([0b00010010].as_ref(), 0),
                code: ErrorKind::Eof
            }))
        );
    }
}
