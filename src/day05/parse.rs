use regex::Regex;
use std::num::ParseIntError;
use thiserror::Error;

use super::{Line, Point, Sequence};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid line {0}")]
    InvalidLine(String),

    #[error("Invalid number {0}")]
    InvalidNumber(#[from] ParseIntError),
}

impl TryFrom<&str> for Line {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"^(\d+),(\d+)\s->\s(\d+),(\d+)$").unwrap(); // TODO lazy_static
        let cap = re
            .captures(value)
            .ok_or_else(|| Error::InvalidLine(value.into()))?;

        Ok(Line {
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
        Ok(Sequence(
            value
                .lines()
                .map(Line::try_from)
                .collect::<Result<Vec<Line>, Self::Error>>()?,
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::day05::{test::EXAMPLE_INPUT, Line, Point, Sequence};

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

    #[test]
    fn parses_example_input() {
        let sequence = Sequence::try_from(EXAMPLE_INPUT).unwrap();
        dbg!(sequence);
    }
}
