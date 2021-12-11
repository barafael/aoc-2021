use itertools::Itertools;

use super::tag::Tag;

#[derive(Debug, Clone, PartialEq)]
pub enum LineStatus {
    Incomplete(Vec<Tag>),
    Corrupt(Tag),
    Correct,
}

pub fn inspect(line: &[Tag]) -> LineStatus {
    let mut stack = vec![];
    for tag in line {
        if tag.is_opening() {
            stack.push(tag);
        } else {
            let opening = stack.pop();
            if let Some(opening) = opening {
                if !opening.matches(tag) {
                    return LineStatus::Corrupt(*tag);
                }
            } else {
                return LineStatus::Correct;
            }
        }
    }
    if stack.is_empty() {
        LineStatus::Correct
    } else {
        let completion = stack.iter().rev().map(|t| t.pair()).collect_vec();
        LineStatus::Incomplete(completion)
    }
}

#[cfg(test)]
#[cfg(feature = "non_solution_test")]
mod test {
    use crate::day10::inspect::LineStatus;
    use crate::day10::tag::Tag;
    use crate::day10::{inspect::inspect, parse_navigation_program};

    #[test]
    fn detects_close_curly_brace() {
        use crate::day10::{inspect::LineStatus, tag::Tag};

        let line = "{([(<{}[<>[]}>{[]{[(<()>";
        let program = parse_navigation_program(line);
        let result = inspect(&program[0]);
        assert_eq!(LineStatus::Corrupt(Tag::CloseCurlyBrace), result);
    }

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

    #[test]
    fn detects_close_square_bracket() {
        let line = "[{[{({}]{}}([{[{{{}}([]";
        let program = parse_navigation_program(line);
        let result = inspect(&program[0]);
        assert_eq!(LineStatus::Corrupt(Tag::CloseSquareBracket), result);
    }

    #[test]
    fn detects_close_angle_bracket() {
        let line = "<{([([[(<>()){}]>(<<{{";
        let program = parse_navigation_program(line);
        let result = inspect(&program[0]);
        assert_eq!(LineStatus::Corrupt(Tag::CloseAngleBracket), result);
    }
}
