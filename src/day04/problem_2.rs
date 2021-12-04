#[cfg(test)]
mod tests {
    use crate::day04::{game::Game, test::INPUT, EXAMPLE_INPUT};

    #[test]
    fn computes_basic_input() {
        let mut game = Game::try_from(EXAMPLE_INPUT).unwrap();
        for num in game.draw_sequence.clone() {
            for board in &mut game.boards {
                board.feed(num);
            }
            if let Some(loser) = game.prune_winning_boards() {
                assert_eq!(1924, loser.sum_unmarked() * num as usize);
                return;
            }
        }
        assert!(false);
    }

    #[test]
    fn computes_result() {
        let mut game = Game::try_from(INPUT).unwrap();
        for num in game.draw_sequence.clone() {
            for board in &mut game.boards {
                board.feed(num);
            }
            if let Some(loser) = game.prune_winning_boards() {
                assert_eq!(3178, loser.sum_unmarked() * num as usize);
                return;
            }
        }
        assert!(false);
    }
}
