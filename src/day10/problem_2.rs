use super::tag::Tag;

pub const fn get_score(tag: &Tag) -> Option<usize> {
    use Tag::*;
    match tag {
        OpenRoundParenthesis | OpenSquareBracket | OpenCurlyBrace | OpenAngleBracket => None,
        CloseParenthesis => Some(1),
        CloseSquareBracket => Some(2),
        CloseCurlyBrace => Some(3),
        CloseAngleBracket => Some(4),
    }
}

pub fn calculate_score(completion: &[Tag]) -> usize {
    let mut score = 0;
    for tag in completion {
        score *= 5;
        score += get_score(tag).unwrap();
    }
    score
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use crate::day10::{
        inspect::{inspect, LineStatus},
        parse_navigation_program,
        tag::Tag,
        test::{EXAMPLE_INPUT, INPUT},
    };
    use super::calculate_score;

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn makes_completion_sequence_1() {
        let input = "[({(<(())[]>[[{[]{<()<>>";
        let program = parse_navigation_program(input);
        let completion = inspect(&program[0]);
        assert_eq!(
            LineStatus::Incomplete(vec![
                Tag::CloseCurlyBrace,
                Tag::CloseCurlyBrace,
                Tag::CloseSquareBracket,
                Tag::CloseSquareBracket,
                Tag::CloseParenthesis,
                Tag::CloseCurlyBrace,
                Tag::CloseParenthesis,
                Tag::CloseSquareBracket,
            ],),
            completion
        );
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn makes_completion_sequence_2() {
        let input = "[(()[<>])]({[<{<<[]>>(";
        let program = parse_navigation_program(input);
        let completion = inspect(&program[0]);
        assert_eq!(
            LineStatus::Incomplete(vec![
                Tag::CloseParenthesis,
                Tag::CloseCurlyBrace,
                Tag::CloseAngleBracket,
                Tag::CloseSquareBracket,
                Tag::CloseCurlyBrace,
                Tag::CloseParenthesis,
            ],),
            completion
        );
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn makes_completion_sequence_3() {
        let input = "(((({<>}<{<{<>}{[]{[]{}";
        let program = parse_navigation_program(input);
        let completion = inspect(&program[0]);
        assert_eq!(
            LineStatus::Incomplete(vec![
                Tag::CloseCurlyBrace,
                Tag::CloseCurlyBrace,
                Tag::CloseAngleBracket,
                Tag::CloseCurlyBrace,
                Tag::CloseAngleBracket,
                Tag::CloseParenthesis,
                Tag::CloseParenthesis,
                Tag::CloseParenthesis,
                Tag::CloseParenthesis
            ]),
            completion
        );
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn makes_completion_sequence_4() {
        let input = "{<[[]]>}<{[{[{[]{()[[[]";
        let program = parse_navigation_program(input);
        let completion = inspect(&program[0]);
        assert_eq!(
            LineStatus::Incomplete(vec![
                Tag::CloseSquareBracket,
                Tag::CloseSquareBracket,
                Tag::CloseCurlyBrace,
                Tag::CloseCurlyBrace,
                Tag::CloseSquareBracket,
                Tag::CloseCurlyBrace,
                Tag::CloseSquareBracket,
                Tag::CloseCurlyBrace,
                Tag::CloseAngleBracket
            ]),
            completion
        );
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn makes_completion_sequence_5() {
        let input = "<{([{{}}[<[[[<>{}]]]>[]]";
        let program = parse_navigation_program(input);
        let completion = inspect(&program[0]);
        assert_eq!(
            LineStatus::Incomplete(vec![
                Tag::CloseSquareBracket,
                Tag::CloseParenthesis,
                Tag::CloseCurlyBrace,
                Tag::CloseAngleBracket
            ]),
            completion
        );
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn calculates_score() {
        let input = vec![
            Tag::CloseSquareBracket,
            Tag::CloseParenthesis,
            Tag::CloseCurlyBrace,
            Tag::CloseAngleBracket,
        ];
        let score = calculate_score(&input);
        assert_eq!(294, score);
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn calculates_example_result() {
        let input = parse_navigation_program(EXAMPLE_INPUT);
        let score = input
            .iter()
            .map(|line| inspect(line))
            .filter_map(|status| {
                if let LineStatus::Incomplete(v) = status {
                    Some(v)
                } else {
                    None
                }
            })
            .map(|completion| calculate_score(&completion))
            .sorted()
            .collect::<Vec<_>>();
        let mid = score[score.len() / 2];
        assert_eq!(288957, mid);
    }

    #[test]
    fn calculates_result() {
        let input = parse_navigation_program(INPUT);
        let score = input
            .iter()
            .map(|line| inspect(line))
            .filter_map(|status| {
                if let LineStatus::Incomplete(v) = status {
                    Some(v)
                } else {
                    None
                }
            })
            .map(|completion| calculate_score(&completion))
            .sorted()
            .collect::<Vec<_>>();
        let mid = score[score.len() / 2];
        assert_eq!(2289754624, mid);
    }
}
