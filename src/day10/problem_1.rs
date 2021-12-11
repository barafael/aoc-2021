#[cfg(test)]
mod tests {
    #[cfg(feature = "non_solution_test")]
    use crate::day10::test::EXAMPLE_INPUT;

    use crate::day10::{
        inspect::{inspect, LineStatus},
        parse_navigation_program,
        test::INPUT,
    };

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn computes_example_program() {
        let program = parse_navigation_program(EXAMPLE_INPUT);
        let sum: u64 = program
            .iter()
            .filter_map(|l| {
                if let LineStatus::Corrupt(tag) = inspect(l) {
                    Some(tag)
                } else {
                    None
                }
            })
            .map(|t| t.get_score())
            .sum();
        assert_eq!(26397, sum);
    }

    #[test]
    fn calculates_solution() {
        let program = parse_navigation_program(INPUT);
        let sum: u64 = program
            .iter()
            .filter_map(|l| {
                if let LineStatus::Corrupt(tag) = inspect(l) {
                    Some(tag)
                } else {
                    None
                }
            })
            .map(|t| t.get_score())
            .sum();
        assert_eq!(339411, sum);
    }
}
