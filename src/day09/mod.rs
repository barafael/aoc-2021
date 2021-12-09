pub mod problem_1;
pub mod problem_2;

pub fn parse_to_vec_vec(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).map(|n| n as u8))
                .collect::<Option<Vec<_>>>()
                .unwrap()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::parse_to_vec_vec;

    pub const INPUT: &str = include_str!("../../input/day09.txt");
    pub const EXAMPLE_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn parses_input_ok() {
        let _input = parse_to_vec_vec(INPUT);
    }
}
