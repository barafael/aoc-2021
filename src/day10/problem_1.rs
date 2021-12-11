#[cfg(test)]
mod tests {
    use crate::day10::{
        inspect::{inspect, LineStatus},
        parse_navigation_program,
        test::{EXAMPLE_INPUT, INPUT},
        Tag,
    };

    // TODO move some tests to inspect.rs
    #[cfg(feature = "non_solution_test")]
    #[test]
    fn detects_close_curly_brace() {
        let line = "{([(<{}[<>[]}>{[]{[(<()>";
        let program = parse_navigation_program(line);
        let result = inspect(&program[0]);
        assert_eq!(LineStatus::Corrupt(Tag::CloseCurlyBrace), result);
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn detects_close_parenthesis() {
        let line = "[[<[([]))<([[{}[[()]]]";
        let program = parse_navigation_program(line);
        let result = inspect(&program[0]);
        assert_eq!(LineStatus::Corrupt(Tag::CloseParenthesis), result);
        let line = "[<(<(<(<{}))><([]([]()";
        let program = parse_navigation_program(line);
        let result = inspect(&program[0]);
        assert_eq!(LineStatus::Corrupt(Tag::CloseParenthesis), result);
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn detects_close_square_bracket() {
        let line = "[{[{({}]{}}([{[{{{}}([]";
        let program = parse_navigation_program(line);
        let result = inspect(&program[0]);
        assert_eq!(LineStatus::Corrupt(Tag::CloseSquareBracket), result);
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn detects_close_angle_bracket() {
        let line = "<{([([[(<>()){}]>(<<{{";
        let program = parse_navigation_program(line);
        let result = inspect(&program[0]);
        assert_eq!(LineStatus::Corrupt(Tag::CloseAngleBracket), result);
    }

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
