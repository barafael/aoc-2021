use self::tag::Tag;

pub mod inspect;
pub mod problem_1;
pub mod problem_2;
pub mod tag;

pub fn parse_navigation_program(input: &str) -> Vec<Vec<Tag>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(Tag::try_from)
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    #[cfg(feature = "non_solution_test")]
    use super::parse_navigation_program;

    pub const INPUT: &str = include_str!("../../input/day10.txt");

    #[cfg(feature = "non_solution_test")]
    pub const EXAMPLE_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn parses_example_input() {
        let _input = parse_navigation_program(EXAMPLE_INPUT);
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn parses_input() {
        let _input = parse_navigation_program(INPUT);
    }
}
