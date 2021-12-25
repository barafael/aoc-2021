use itertools::Itertools;

pub fn try_parse(input: &[u8]) -> Option<(u64, &[u8])> {
    let mut literal_vec = vec![];
    for chunk in &input.iter().chunks(5) {
        let vec = chunk.copied().collect_vec();
        literal_vec.extend_from_slice(&vec[1..]);
        if vec[0] == 0 {
            break;
        }
    }
    let mut literal = 0u64;
    assert!(literal_vec.len() <= 64);
    // 'keep going' bits and literal bits.
    let len = literal_vec.len() / 4 + literal_vec.len();
    for (index, bit) in literal_vec.into_iter().rev().enumerate() {
        if bit != 0 {
            literal |= 1 << index;
        }
    }
    Some((literal, &input[len..]))
}

#[cfg(test)]
mod tests {
    use super::try_parse;

    #[test]
    fn parses_literal() {
        let input = vec![1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0];
        let (literal, remainder) = try_parse(&input).unwrap();
        assert_eq!(2021, literal);
        assert_eq!(&[0, 0, 0], remainder);
    }
}
