pub mod problem_1;
pub mod problem_2;

pub fn parse_octopi(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).map(|n| n as u8))
                .collect::<Option<Vec<u8>>>()
                .unwrap()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::parse_octopi;

    pub const INPUT: &str = include_str!("../../input/day11.txt");

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn parses_input() {
        let _input = parse_octopi(INPUT);
    }
}
