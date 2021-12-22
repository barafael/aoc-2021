#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::day21::{player::Player, test::INPUT};

    #[test]
    fn computes_solution() {
        let lines = &mut INPUT.lines();
        let player_1 = Player::from_str(lines.next().unwrap()).unwrap();
        let player_2 = Player::from_str(lines.next().unwrap()).unwrap();

        let mut players = [player_1, player_2];

        let mut dice = 1;
        let mut throwed = 0;
        let mut next_dice = || {
            let o = dice;
            dice = dice % 100 + 1;
            throwed += 1;
            o
        };
        let value = loop {
            let mut highscore = 0;
            for (i, mut p) in players.iter_mut().enumerate() {
                p.position += next_dice() + next_dice() + next_dice();
                p.position = (p.position - 1) % 10 + 1;
                p.score += p.position;
                if p.score >= 1000 {
                    highscore = players[1 - i].score as u32;
                    break;
                }
            }
            if highscore > 0 {
                break highscore;
            }
        };
        assert_eq!(908595, value * throwed);
    }
}
