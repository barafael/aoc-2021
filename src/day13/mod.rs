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
}

pub fn parse_manual(s: &str) -> (UserManual, Vec<Fold>) {
    let mut manual = UserManual::default();
    let mut instructions = vec![];
    for line in s.lines() {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("fold") {
            let (c, n) = line
                .split_whitespace()
                .nth(2)
                .unwrap()
                .split_once('=')
                .unwrap();
            match c {
                "x" => {
                    instructions.push(Fold::Vertical(n.parse().unwrap()));
                }
                "y" => instructions.push(Fold::Horizontal(n.parse().unwrap())),
                _ => unreachable!(),
            }
        } else {
            let (x, y) = line.split_once(',').unwrap();
            manual
                .coordinates
                .push((x.parse().unwrap(), y.parse().unwrap()));
        }
    }
    (manual, instructions)
}

impl UserManual {
    pub fn apply_fold(&mut self, fold: &Fold) {
        match fold {
            Fold::Horizontal(n) => self.fold_up(*n),
            Fold::Vertical(n) => self.fold_left(*n),
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
        self.coordinates = new_coordinates;
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
        self.coordinates = new_coordinates;
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
    #[cfg(feature = "non_solution_test")]
    use super::UserManual;
    #[cfg(feature = "non_solution_test")]
    use crate::day13::{parse_manual, Fold};

    pub const INPUT: &str = include_str!("../../input/day13.txt");

    #[cfg(feature = "non_solution_test")]
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

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn parses_input() {
        let (manual, instructions) = parse_manual(EXAMPLE_INPUT);
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
        };
        let expected_instructions = vec![Fold::Horizontal(7), Fold::Vertical(5)];
        assert_eq!(expected, manual);
        assert_eq!(expected_instructions, instructions);
    }

    #[cfg(feature = "non_solution_test")]
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
        };
        manual.apply_fold(&Fold::Horizontal(7));
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
        };
        assert_eq!(expected, manual);
    }

    #[cfg(feature = "non_solution_test")]
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
        };
        manual.apply_fold(&Fold::Vertical(5));
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
        };
        assert_eq!(expected, manual);
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn formats_example_input() {
        let (manual, _instructions) = parse_manual(EXAMPLE_INPUT);
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
