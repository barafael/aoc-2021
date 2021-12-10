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
