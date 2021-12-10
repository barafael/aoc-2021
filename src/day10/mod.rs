use self::tag::Tag;

pub mod problem_1;
pub mod problem_2;
pub mod tag;
pub mod inspect;

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
    use super::parse_navigation_program;

    pub const INPUT: &str = include_str!("../../input/day10.txt");
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

    #[test]
    fn parses_example_input() {
        let _input = parse_navigation_program(EXAMPLE_INPUT);
    }

    #[test]
    fn parses_input() {
        let _input = parse_navigation_program(INPUT);
    }
}
