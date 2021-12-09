#[cfg(test)]
mod tests {
    use crate::day09::{
        find_low_points, parse_to_vec_vec,
        test::{EXAMPLE_INPUT, INPUT},
    };

    #[test]
    fn calculates_sum_of_risk_levels_example_data() {
        let input = parse_to_vec_vec(EXAMPLE_INPUT);
        let sum = find_low_points(&input)
            .iter()
            .map(|(x, y)| input[*x][*y] as u64 + 1)
            .sum();
        assert_eq!(15u64, sum);
    }

    #[test]
    fn calculates_sum_of_risk_levels() {
        let input = parse_to_vec_vec(INPUT);
        let sum = find_low_points(&input)
            .iter()
            .map(|(x, y)| input[*x][*y] as u64 + 1)
            .sum();
        assert_eq!(539u64, sum);
    }
}
