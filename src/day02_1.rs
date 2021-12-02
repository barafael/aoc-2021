use std::num::ParseIntError;

use regex::Regex;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Debug, Default)]
pub struct Position {
    x: i64,
    y: i64,
}

pub fn navigate_by_course(course: &[Command]) -> Position {
    let mut position = Position::default();
    for command in course {
        match command {
            Command::Forward(n) => position.x += i64::from(*n),
            Command::Down(n) => position.y += i64::from(*n),
            Command::Up(n) => position.y -= i64::from(*n),
        }
    }
    position
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid command {0}")]
    InvalidCommand(String),

    #[error("Invalid number {0}")]
    InvalidNumber(#[source] ParseIntError),
}

pub fn parse_course_from_str(str: &str) -> Result<Vec<Command>, ParseError> {
    let re = Regex::new(r"(forward|up|down)\s+(\d+)").unwrap();
    let mut course = vec![];
    for line in str.lines() {
        let cap = re
            .captures(line)
            .ok_or_else(|| ParseError::InvalidCommand(line.into()))?;
        match (cap.get(1), cap.get(2)) {
            (Some(command), Some(n)) => {
                let n = n
                    .as_str()
                    .parse::<u32>()
                    .map_err(ParseError::InvalidNumber)?;
                let command = match command.as_str() {
                    "forward" => Command::Forward(n),
                    "down" => Command::Down(n),
                    "up" => Command::Up(n),
                    s => unreachable!("Found some kind of mean edge case: {}", s),
                };
                course.push(command);
            }
            _ => return Err(ParseError::InvalidCommand(line.into())),
        }
    }
    Ok(course)
}

#[cfg(test)]
mod tests {
    use super::{navigate_by_course, parse_course_from_str, Command};

    const INPUT: &str = include_str!("../input/day02.txt");

    #[test]
    fn basic_input_results_in_15() {
        let commands = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];
        let final_position = navigate_by_course(&commands);
        assert_eq!(final_position.x * final_position.y, 150);
    }

    #[test]
    fn parses_sample_str() {
        let course = parse_course_from_str(
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
        assert!(parse_course_from_str(INPUT).is_ok());
    }

    #[test]
    fn computes_result() {
        let course = parse_course_from_str(INPUT).unwrap();
        let final_position = navigate_by_course(&course);
        assert_eq!(1924923, final_position.x * final_position.y);
    }
}
