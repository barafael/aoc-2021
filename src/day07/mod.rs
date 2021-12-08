use std::num::ParseIntError;

pub mod problem_1;

pub fn parse_to_vec(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<usize>, _>>()
}

#[cfg(test)]
mod test {
    use super::parse_to_vec;

    pub const INPUT: &str = include_str!("../../input/day07.txt");
    pub const EXAMPLE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn parses_input_ok() {
        assert!(parse_to_vec(INPUT).is_ok());
    }
}
