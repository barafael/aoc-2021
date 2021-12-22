pub mod player;
pub mod problem_1;
pub mod problem_2;
pub mod situation;

#[cfg(test)]
mod test {
    use super::player::Player;
    use std::str::FromStr;

    pub const INPUT: &str = include_str!("../../input/day21.txt");

    #[test]
    fn parses_input_ok() {
        let lines = &mut INPUT.lines();
        let player_1 = Player::from_str(lines.next().unwrap()).unwrap();
        let player_2 = Player::from_str(lines.next().unwrap()).unwrap();
        assert_eq!(Player::new(4), player_1);
        assert_eq!(Player::new(2), player_2);
    }
}
