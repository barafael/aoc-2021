use super::count_ones_in_all_columns;

pub fn calculate_power_consumption(patterns: &[u16], width: usize) -> usize {
    let counter = count_ones_in_all_columns(patterns, width);
    let mut gamma = 0;
    for (index, c) in counter.iter().enumerate() {
        if *c > patterns.len() / 2 {
            gamma |= 1 << index;
        }
    }
    let epsilon = !gamma & ((1 << width) - 1);
    gamma * epsilon
}

#[cfg(test)]
mod tests {
    use super::calculate_power_consumption;
    use crate::day03::{INPUT, parse_from_str};

    #[test]
    fn computes_basic_input() {
        let numbers = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        let consumption = calculate_power_consumption(&numbers, 5);
        assert_eq!(198, consumption);
    }

    #[test]
    fn computes_result() {
        let input = parse_from_str(INPUT);
        let result = calculate_power_consumption(&input, 12);
        assert_eq!(3912944, result);
    }
}
