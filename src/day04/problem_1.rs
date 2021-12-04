#[cfg(test)]
mod test {
    use crate::day04::{
        game::Game,
        test::{EXAMPLE_INPUT, INPUT},
    };

    #[test]
    fn computes_basic_input() {
        let mut game = Game::try_from(EXAMPLE_INPUT).unwrap();
        for num in game.draw_sequence {
            for board in &mut game.boards {
                if board.feed(num) {
                    assert_eq!(4512, board.sum_unmarked() * num as usize);
                    return;
                }
            }
        }
        assert!(false);
    }

    #[test]
    fn computes_solution() {
        let mut game = Game::try_from(INPUT).unwrap();
        for num in game.draw_sequence {
            for board in &mut game.boards {
                if board.feed(num) {
                    assert_eq!(41503, board.sum_unmarked() * num as usize);
                    return;
                }
            }
        }
        assert!(false);
    }
}
