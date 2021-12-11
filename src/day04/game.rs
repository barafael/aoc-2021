use super::board::Board;
use itertools::Itertools;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse board {0}")]
    ParseBoard(#[source] super::board::Error),

    #[error("Initial line is required")]
    InitialLineRequired,

    #[error("Failed to parse number on initial line with error {0}")]
    ParseNumberOnInitialLine(#[from] ParseIntError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub draw_sequence: Vec<u8>,
    pub boards: Vec<Board>,
}

impl Game {
    pub fn prune_winning_boards(&mut self) -> Option<Board> {
        if self.boards.len() == 1 && self.boards[0].win_condition() {
            return Some(self.boards[0].clone());
        }
        let boards = self
            .boards
            .iter()
            .filter(|b| !b.win_condition())
            .cloned()
            .collect::<Vec<_>>();
        self.boards = boards;
        None
    }
}

impl TryFrom<&str> for Game {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
        let initial = lines.next().ok_or(Error::InitialLineRequired)?;
        let initial: Vec<u8> = initial
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, ParseIntError>>()?;

        let chungus = lines.chunks(6);
        let chungus = chungus.into_iter().map(|s| s.skip(1));

        let mut boards = vec![];
        for board_lines in chungus {
            let t = board_lines.collect::<Vec<_>>();
            let board = Board::try_from(t).map_err(Error::ParseBoard)?;
            boards.push(board);
        }
        Ok(Self {
            draw_sequence: initial,
            boards,
        })
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "non_solution_test")]
    use super::Game;

    #[cfg(feature = "non_solution_test")]
    use crate::day04::{
        board::Board,
        test::{EXAMPLE_INPUT, INPUT},
    };

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn parses_game() {
        let example = Game::try_from(EXAMPLE_INPUT).unwrap();
        let example_game: Game = Game {
            draw_sequence: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
            boards: vec![
                Board {
                    numbers: [
                        [22, 13, 17, 11, 0],
                        [8, 2, 23, 4, 24],
                        [21, 9, 14, 16, 7],
                        [6, 10, 3, 18, 5],
                        [1, 12, 20, 15, 19],
                    ],
                    marks: [[false; 5]; 5],
                },
                Board {
                    numbers: [
                        [3, 15, 0, 2, 22],
                        [9, 18, 13, 17, 5],
                        [19, 8, 7, 25, 23],
                        [20, 11, 10, 24, 4],
                        [14, 21, 16, 12, 6],
                    ],
                    marks: [[false; 5]; 5],
                },
                Board {
                    numbers: [
                        [14, 21, 17, 24, 4],
                        [10, 16, 15, 9, 19],
                        [18, 8, 23, 26, 20],
                        [22, 11, 13, 6, 5],
                        [2, 0, 12, 3, 7],
                    ],
                    marks: [[false; 5]; 5],
                },
            ],
        };

        assert_eq!(example_game, example);
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn parses_input_ok() {
        let _example = Game::try_from(INPUT).unwrap();
    }
}
