use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot parse board from {0}")]
    ParseBoard(String),

    #[error("Cannot parse number with error {0}")]
    ParseNumber(#[from] ParseIntError),

    #[error("Row count {0} invalid, must be 5")]
    InvalidRowCount(usize),

    #[error("Column count {0} invalid, must be 5")]
    InvalidColumnCount(usize),
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Board {
    pub(super) numbers: [[u8; 5]; 5],
    pub(super) marks: [[bool; 5]; 5],
}

impl Board {
    /// Returns true if board wins after feeding `num`.
    pub fn feed(&mut self, num: u8) -> bool {
        self.mark(num);
        self.win_condition()
    }

    pub fn sum_unmarked(&self) -> usize {
        let mut sum = 0usize;
        for row in 0..5 {
            for col in 0..5 {
                if !self.marks[row][col] {
                    sum += self.numbers[row][col] as usize;
                }
            }
        }
        sum
    }

    fn mark(&mut self, num: u8) {
        for i in 0..5 {
            for j in 0..5 {
                if self.numbers[i][j] == num {
                    self.marks[i][j] = true;
                }
            }
        }
    }

    pub fn win_condition(&self) -> bool {
        self.has_row() || self.has_column()
    }

    fn has_row(&self) -> bool {
        for row in self.marks {
            if row == [true; 5] {
                return true;
            }
        }
        false
    }

    fn has_column(&self) -> bool {
        for col in 0..5 {
            let mut count = 0;
            for row in 0..5 {
                if self.marks[row][col] {
                    count += 1;
                }
            }
            if count == 5 {
                return true;
            }
        }
        false
    }
}

impl TryFrom<&str> for Board {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut board = Self {
            numbers: [[0u8; 5]; 5],
            marks: [[false; 5]; 5],
        };
        let lines: Vec<_> = value.lines().collect();
        if lines.len() != 5 {
            return Err(Error::InvalidRowCount(lines.len()));
        }
        for (index, line) in lines.iter().enumerate() {
            let line = line
                .split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<u8>, ParseIntError>>()?;
            if line.len() != 5 {
                return Err(Error::InvalidColumnCount(line.len()));
            }
            board.numbers[index].copy_from_slice(&line);
        }
        Ok(board)
    }
}

impl TryFrom<Vec<&str>> for Board {
    type Error = Error;

    fn try_from(lines: Vec<&str>) -> Result<Self, Self::Error> {
        let mut board = Self {
            numbers: [[0u8; 5]; 5],
            marks: [[false; 5]; 5],
        };
        if lines.len() != 5 {
            return Err(Error::InvalidRowCount(lines.len()));
        }
        for (index, line) in lines.iter().enumerate() {
            let line = line
                .split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<u8>, ParseIntError>>()?;
            if line.len() != 5 {
                return Err(Error::InvalidColumnCount(line.len()));
            }
            board.numbers[index].copy_from_slice(&line);
        }
        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use super::Board;

    const EXAMPLE_BOARD_STR: &str = r"22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19";
    const EXAMPLE_BOARD: Board = Board {
        numbers: [
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ],
        marks: [[false; 5]; 5],
    };

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn marks_numbers() {
        let mut board = EXAMPLE_BOARD;
        board.mark(13);
        assert!(board.marks[0][1]);
        board.numbers[0][0] = 11;
        board.mark(11);
        assert!(board.marks[0][0]);
        assert!(board.marks[0][3]);
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn detects_rows() {
        let mut board = EXAMPLE_BOARD;
        assert!(!board.has_row());
        assert!(!board.has_column());
        board.marks[3] = [true; 5];
        assert!(board.has_row());
        assert!(!board.has_column());
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn detects_columns() {
        let mut board = EXAMPLE_BOARD;
        assert!(!board.has_column());
        assert!(!board.has_row());
        board.marks[0][2] = true;
        board.marks[1][2] = true;
        board.marks[2][2] = true;
        board.marks[3][2] = true;
        board.marks[4][2] = true;
        assert!(board.has_column());
        assert!(!board.has_row());
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn detects_win() {
        let mut board = EXAMPLE_BOARD;
        let bools = [8, 1, 24, 2, 23, 4, 10, 12, 2]
            .iter()
            .map(|n| board.feed(*n))
            .collect::<Vec<_>>();
        assert_eq!(
            vec![false, false, false, false, false, true, true, true, true],
            bools
        );
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn sums_up_unmarked() {
        let board = EXAMPLE_BOARD;
        assert_eq!(300, board.sum_unmarked());
        let mut board = board;
        assert!(!board.feed(20));
        assert_eq!(280, board.sum_unmarked());
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn parses_board_from_basic_example_str() {
        let board = Board::try_from(EXAMPLE_BOARD_STR);
        assert_eq!(EXAMPLE_BOARD, board.unwrap());

        let vec = EXAMPLE_BOARD_STR.lines().collect::<Vec<_>>();

        let board = Board::try_from(vec);
        assert_eq!(EXAMPLE_BOARD, board.unwrap());
    }
}
