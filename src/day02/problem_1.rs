use super::{Command, Position};

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

#[cfg(test)]
mod tests {
    use super::{navigate_by_course, Command};
    use crate::day02::{parse::course_from_str, INPUT};

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
    fn computes_result() {
        let course = course_from_str(INPUT).unwrap();
        let final_position = navigate_by_course(&course);
        assert_eq!(1924923, final_position.x * final_position.y);
    }
}
