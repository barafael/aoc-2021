use super::number::Number;

pub fn parse_input(input: &str) -> Vec<Number> {
    input
        .lines()
        .filter_map(|l| serde_json::from_str::<Number>(l).ok())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::parse_input;
    use crate::day18::test::{EXAMPLE, INPUT};
    use itertools::Itertools;

    #[test]
    fn parses_example_ok() {
        let numbers = parse_input(EXAMPLE);
        assert_eq!(EXAMPLE.trim(), format!("{}", numbers.iter().format("\n")));
    }

    #[test]
    fn parses_input_ok() {
        let numbers = parse_input(INPUT);
        assert_eq!(INPUT.trim(), format!("{}", numbers.iter().format("\n")));
    }
}
