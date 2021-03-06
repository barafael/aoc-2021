use super::Command;
use lazy_static::lazy_static;
use regex::Regex;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid command {0}")]
    InvalidCommand(String),

    #[error("Invalid number {0}")]
    InvalidNumber(#[source] ParseIntError),
}

lazy_static! {
    static ref COMMAND_REGEX: Regex = Regex::new(r"^(forward|up|down)\s+(\d+)$").unwrap();
}

pub fn course_from_str(str: &str) -> Result<Vec<Command>, Error> {
    let mut course = vec![];
    for line in str.lines() {
        let cap = COMMAND_REGEX
            .captures(line)
            .ok_or_else(|| Error::InvalidCommand(line.into()))?;
        match (cap.get(1), cap.get(2)) {
            (Some(command), Some(n)) => {
                let n = n.as_str().parse::<u32>().map_err(Error::InvalidNumber)?;
                let command = match command.as_str() {
                    "forward" => Command::Forward(n),
                    "down" => Command::Down(n),
                    "up" => Command::Up(n),
                    s => unreachable!("Found some kind of mean edge case: {}", s),
                };
                course.push(command);
            }
            _ => return Err(Error::InvalidCommand(line.into())),
        }
    }
    Ok(course)
}

#[cfg(feature = "non_solution_test")]
#[cfg(test)]
mod test {
    use crate::day02::{
        parse::{course_from_str, Error},
        Command, INPUT,
    };

    #[test]
    fn parses_sample_str() {
        let course = course_from_str(
            r##"forward 30
up 2
down 1
forward 150
down 2
up 234"##,
        )
        .unwrap();
        let expected = vec![
            Command::Forward(30),
            Command::Up(2),
            Command::Down(1),
            Command::Forward(150),
            Command::Down(2),
            Command::Up(234),
        ];
        assert_eq!(expected, course);
    }

    #[test]
    fn parses_input_str_ok() {
        assert!(course_from_str(INPUT).is_ok());
    }

    #[test]
    fn accepts_empty_input() {
        assert_eq!(course_from_str("").unwrap(), vec![]);
    }

    #[test]
    fn rejects_invalid_input() {
        assert!(
            matches!(course_from_str("forward 30,").unwrap_err(), Error::InvalidCommand(c) if c == "forward 30,".to_string())
        );
        assert!(
            matches!(course_from_str("Up 300").unwrap_err(), Error::InvalidCommand(c) if c == "Up 300".to_string())
        );
        assert!(
            matches!(course_from_str("down -14").unwrap_err(), Error::InvalidCommand(c) if c == "down -14".to_string())
        );
    }
}
