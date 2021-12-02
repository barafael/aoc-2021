use super::{Command, Position};

pub fn navigate_by_course(course: &[Command]) -> Position {
    let mut position = Position::default();
    let mut aim = 0;
    for command in course {
        match command {
            Command::Down(n) => aim += i64::from(*n),
            Command::Up(n) => aim -= i64::from(*n),
            Command::Forward(n) => {
                position.x += i64::from(*n);
                position.y += aim * i64::from(*n);
            }
        }
    }
    position
}

#[cfg(test)]
mod tests {
    use crate::day02::{parse, problem_2::navigate_by_course, Command, INPUT};

    #[test]
    fn basic_input_results_in_900() {
        let course = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];
        let final_position = navigate_by_course(&course);
        assert_eq!(final_position.x * final_position.y, 900);
    }

    #[test]
    fn computes_result() {
        let course = parse::course_from_str(INPUT).unwrap();
        let final_position = navigate_by_course(&course);
        assert_eq!(1982495697, final_position.x * final_position.y);
    }
}
