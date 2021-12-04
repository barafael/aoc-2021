pub mod problem_1;
pub mod problem_2;

const fn bit_is_set(num: u16, index: usize) -> bool {
    if index > 15 {
        return false;
    }
    (num & (1 << index)) != 0
}

pub fn count_ones_in_column(patterns: &[u16], position: usize) -> usize {
    let mut counter = 0;
    for pattern in patterns {
        if bit_is_set(*pattern, position) {
            counter += 1;
        }
    }
    counter
}

pub fn count_ones_in_all_columns(patterns: &[u16], width: usize) -> Vec<usize> {
    let mut counter = vec![0; width];
    for (index, elem) in counter.iter_mut().enumerate() {
        *elem = count_ones_in_column(patterns, index);
    }
    counter
}

pub fn parse_from_str(s: &str) -> Vec<u16> {
    s.lines()
        .map(|l| u16::from_str_radix(l, 2).unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day03::{count_ones_in_all_columns, count_ones_in_column, parse_from_str};

    pub const INPUT: &str = include_str!("../../input/day03.txt");

    #[test]
    fn counts_ones_in_column() {
        let input = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        let counter = count_ones_in_column(&input, 0);
        assert_eq!(5, counter);

        let counter = count_ones_in_column(&input, 1);
        assert_eq!(7, counter);

        let counter = count_ones_in_column(&input, 2);
        assert_eq!(8, counter);
    }

    #[test]
    fn counts_ones_in_all_columns() {
        let input = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        let counter = count_ones_in_all_columns(&input, 5);
        assert_eq!(vec![5, 7, 8, 5, 7], counter);
    }

    #[test]
    fn parses_basic_input() {
        let basic_input = r##"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"##;
        let numbers = parse_from_str(basic_input);
        let expected = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        assert_eq!(expected, numbers);
    }
}
