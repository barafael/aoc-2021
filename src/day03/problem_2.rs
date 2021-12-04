use super::bit_is_set;
use crate::day03::count_ones_in_column;
use std::cmp::Ordering;

pub fn calculate_oxygen_rating(patterns: &[u16], width: usize) -> usize {
    calculate_param(patterns, width - 1, true)
}

pub fn calculate_co2_rating(patterns: &[u16], width: usize) -> usize {
    calculate_param(patterns, width - 1, false)
}

pub fn calculate_param(patterns: &[u16], position: usize, is_most: bool) -> usize {
    let ones = count_ones_in_column(patterns, position);

    let zeros = patterns.len() - ones;
    let ordering = ones.cmp(&zeros);
    let preserve_with_bit = match ordering {
        Ordering::Less => {
            if is_most {
                0
            } else {
                1
            }
        }
        Ordering::Equal | Ordering::Greater => {
            if is_most {
                1
            } else {
                0
            }
        }
    };
    let remain = filter_position(patterns, position, preserve_with_bit == 1);

    if remain.len() == 1 {
        return remain[0] as usize;
    }

    calculate_param(&remain, position - 1, is_most)
}

fn filter_position(patterns: &[u16], position: usize, is_one: bool) -> Vec<u16> {
    let mut left = vec![];

    for pattern in patterns {
        if bit_is_set(*pattern, position) == is_one {
            left.push(*pattern);
        }
    }
    left
}

#[cfg(test)]
mod tests {
    use super::filter_position;
    use crate::day03::{
        parse_from_str,
        problem_2::{calculate_co2_rating, calculate_oxygen_rating},
        test::INPUT,
    };

    #[test]
    fn splits_by_bit() {
        let input = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        assert_eq!(vec![23, 21, 15, 7, 25], filter_position(&input, 0, true));
        assert_eq!(
            vec![4, 30, 22, 28, 16, 2, 10,],
            filter_position(&input, 0, false)
        );
    }

    #[test]
    fn calculates_basic_oxygen_is_23() {
        let input = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        let oxygen = calculate_oxygen_rating(&input, 5);
        assert_eq!(23, oxygen);
    }

    #[test]
    fn calculates_basic_co2_is_10() {
        let input = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        let co2 = calculate_co2_rating(&input, 5);
        assert_eq!(10, co2);
    }

    #[test]
    fn computes_solution() {
        let input = parse_from_str(INPUT);
        let oxygen = calculate_oxygen_rating(&input, 12);
        let co2 = calculate_co2_rating(&input, 12);
        assert_eq!(4996233, oxygen * co2);
    }
}
