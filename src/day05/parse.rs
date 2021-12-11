use super::{Line, Point, Sequence};
use lazy_static::lazy_static;
use regex::Regex;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid line {0}")]
    InvalidLine(String),

    #[error("Invalid number {0}")]
    InvalidNumber(#[from] ParseIntError),
}

lazy_static! {
    static ref POINT_REGEX: Regex = Regex::new(r"^(\d+),(\d+)\s->\s(\d+),(\d+)$").unwrap();
}

impl TryFrom<&str> for Line {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cap = POINT_REGEX
            .captures(value)
            .ok_or_else(|| Error::InvalidLine(value.into()))?;

        Ok(Self {
            start: Point {
                x: cap.get(1).unwrap().as_str().parse::<usize>()?,
                y: cap.get(2).unwrap().as_str().parse::<usize>()?,
            },
            end: Point {
                x: cap.get(3).unwrap().as_str().parse::<usize>()?,
                y: cap.get(4).unwrap().as_str().parse::<usize>()?,
            },
        })
    }
}

impl TryFrom<&str> for Sequence {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(
            value
                .lines()
                .map(Line::try_from)
                .collect::<Result<Vec<Line>, Self::Error>>()?,
        ))
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "non_solution_test")]
    use crate::day05::{Line, Point, Sequence};

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn parses_sequence() {
        let line = Line::try_from("5,6 -> 7,8").unwrap();
        assert_eq!(
            Line {
                start: Point { x: 5, y: 6 },
                end: Point { x: 7, y: 8 }
            },
            line
        );
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn parses_example_input() {
        let sequence = Sequence::try_from(crate::day05::test::EXAMPLE_INPUT).unwrap();
        dbg!(sequence);
    }
}
