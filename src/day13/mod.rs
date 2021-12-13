use std::str::FromStr;
use itertools::Itertools;

pub mod problem_1;
pub mod problem_2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UserManual {
    coordinates: Vec<(isize, isize)>,
    instructions: Vec<Fold>,
}

impl FromStr for UserManual {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut manual = Self::default();
        for line in s.lines() {
            if line.starts_with("fold") {
                let (c, n) = line
                    .split_whitespace()
                    .nth(2)
                    .unwrap()
                    .split_once('=')
                    .unwrap();
                match c {
                    "x" => {
                        manual.instructions.push(Fold::Vertical(n.parse().unwrap()));
                    }
                    "y" => manual
                        .instructions
                        .push(Fold::Horizontal(n.parse().unwrap())),
                    _ => unreachable!(),
                }
            } else if line.is_empty() {
            } else {
                let (x, y) = line.split_once(',').unwrap();
                manual
                    .coordinates
                    .push((x.parse().unwrap(), y.parse().unwrap()));
            }
        }
        manual.instructions = manual.instructions.iter().rev().cloned().collect();
        Ok(manual)
    }
}

impl UserManual {
    pub fn apply_fold(&mut self) {
        if let Some(fold) = self.instructions.pop() {
            match fold {
                Fold::Horizontal(n) => self.fold_up(n),
                Fold::Vertical(n) => self.fold_left(n),
            }
        }
    }

    fn fold_up(&mut self, n: usize) {
        let mut new_coordinates = vec![];
        for (x, y) in &mut self.coordinates {
            // must be normalized :) else underflow.
            if *y as usize > n {
                new_coordinates.push((*x, n as isize - (*y - n as isize)));
            } else {
                new_coordinates.push((*x, *y));
            }
        }
        let new_coordinates = new_coordinates.into_iter().unique().collect::<Vec<_>>();
        // normalize.
        let min = *new_coordinates.iter().map(|(_, y)| y).min().unwrap();
        let new_coordinates = new_coordinates
            .iter()
            .map(|(x, y)| (*x, y + min))
            .collect::<Vec<_>>();
        self.coordinates = new_coordinates
    }

    fn fold_left(&mut self, n: usize) {
        let mut new_coordinates = vec![];
        for (x, y) in &mut self.coordinates {
            // must be normalized :) else underflow.
            if *x as usize > n {
                new_coordinates.push((n as isize - (*x - n as isize), *y));
            } else {
                new_coordinates.push((*x, *y));
            }
        }
        let new_coordinates = new_coordinates.into_iter().unique().collect::<Vec<_>>();
        // normalize.
        let min = *new_coordinates.iter().map(|(x, _)| x).min().unwrap();
        let new_coordinates = new_coordinates
            .iter()
            .map(|(x, y)| (*x + min, *y))
            .collect::<Vec<_>>();
        self.coordinates = new_coordinates
    }
}

impl std::fmt::Display for UserManual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.coordinates.iter().map(|(x, _y)| x).max().unwrap();
        let depth = self.coordinates.iter().map(|(_x, y)| y).max().unwrap();
        let mut grid = vec![vec!['.'; *width as usize + 1]; *depth as usize + 1];
        for (x, y) in &self.coordinates {
            grid[*y as usize][*x as usize] = '#';
        }
        for line in &grid {
            write!(f, "{}", line.iter().format(""))?;
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::UserManual;
    use crate::day13::Fold;
    use std::str::FromStr;

    pub const INPUT: &str = include_str!("../../input/day13.txt");
    pub const EXAMPLE_INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn parses_input() {
        let manual = UserManual::from_str(EXAMPLE_INPUT).unwrap();
        let expected = UserManual {
            coordinates: vec![
                (6, 10),
                (0, 14),
                (9, 10),
                (0, 3),
                (10, 4),
                (4, 11),
                (6, 0),
                (6, 12),
                (4, 1),
                (0, 13),
                (10, 12),
                (3, 4),
                (3, 0),
                (8, 4),
                (1, 10),
                (2, 14),
                (8, 10),
                (9, 0),
            ],
            instructions: vec![Fold::Vertical(5), Fold::Horizontal(7)],
        };
        assert_eq!(expected, manual);
    }

    #[test]
    fn folds_up() {
        let mut manual = UserManual {
            coordinates: vec![
                (6, 10),
                (0, 14),
                (9, 10),
                (0, 3),
                (10, 4),
                (4, 11),
                (6, 0),
                (6, 12),
                (4, 1),
                (0, 13),
                (10, 12),
                (3, 4),
                (3, 0),
                (8, 4),
                (1, 10),
                (2, 14),
                (8, 10),
                (9, 0),
            ],
            instructions: vec![Fold::Vertical(5), Fold::Horizontal(7)],
        };
        manual.apply_fold();
        let expected = UserManual {
            coordinates: vec![
                (6, 4),
                (0, 0),
                (9, 4),
                (0, 3),
                (10, 4),
                (4, 3),
                (6, 0),
                (6, 2),
                (4, 1),
                (0, 1),
                (10, 2),
                (3, 4),
                (3, 0),
                (8, 4),
                (1, 4),
                (2, 0),
                (9, 0),
            ],
            instructions: vec![Fold::Vertical(5)],
        };
        assert_eq!(expected, manual);
    }

    #[test]
    fn folds_left() {
        let mut manual = UserManual {
            coordinates: vec![
                (6, 10),
                (0, 14),
                (9, 10),
                (0, 3),
                (10, 4),
                (4, 11),
                (6, 0),
                (6, 12),
                (4, 1),
                (0, 13),
                (10, 12),
                (3, 4),
                (3, 0),
                (8, 4),
                (1, 10),
                (2, 14),
                (8, 10),
                (9, 0),
            ],
            instructions: vec![Fold::Horizontal(7), Fold::Vertical(5)],
        };
        manual.apply_fold();
        let expected = UserManual {
            coordinates: vec![
                (4, 10),
                (0, 14),
                (1, 10),
                (0, 3),
                (0, 4),
                (4, 11),
                (4, 0),
                (4, 12),
                (4, 1),
                (0, 13),
                (0, 12),
                (3, 4),
                (3, 0),
                (2, 4),
                (2, 14),
                (2, 10),
                (1, 0),
            ],
            instructions: vec![Fold::Horizontal(7)],
        };
        assert_eq!(expected, manual);
    }

    #[test]
    fn formats_example_input() {
        let manual = UserManual::from_str(EXAMPLE_INPUT).unwrap();
        let expected = "...#..#..#.
....#......
...........
#..........
...#....#.#
...........
...........
...........
...........
...........
.#....#.##.
....#......
......#...#
#..........
#.#........
";
        assert_eq!(expected, format!("{}", manual));
    }
}
