use super::Line;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Map {
    pub(super) entries: Vec<Vec<u16>>,
    height: usize,
    width: usize,
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                write!(
                    f,
                    "{}",
                    if self.entries[i][j] == 0 {
                        ".".into()
                    } else {
                        self.entries[i][j].to_string()
                    }
                )?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            entries: vec![vec![0; width]; height],
            height,
            width,
        }
    }

    pub fn feed(&mut self, l: &Line) {
        if l.start.x == l.end.x {
            let min = usize::min(l.start.y, l.end.y);
            let max = usize::max(l.start.y, l.end.y);
            for y in min..=max {
                self.entries[y][l.start.x] += 1;
            }
        } else if l.start.y == l.end.y {
            let min = usize::min(l.start.x, l.end.x);
            let max = usize::max(l.start.x, l.end.x);
            for x in min..=max {
                self.entries[l.start.y][x] += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Map;
    use crate::day05::{test::EXAMPLE_INPUT, Sequence};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref MAP: Map = Map {
            width: 10,
            height: 10,
            entries: vec![
                vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 1, 1, 2, 1, 1, 1, 2, 1, 1],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
            ],
        };
    }

    #[test]
    fn map_displays() {
        let expected = r##".......1..
..1....1..
..1....1..
.......1..
.112111211
..........
..........
..........
..........
222111....
"##;
        assert_eq!(expected, format!("{}", *MAP))
    }

    #[test]
    fn feeds_vert_line() {
        let mut map = Map::new(10, 10);
        let line = Sequence::try_from(EXAMPLE_INPUT).unwrap().0[0];
        map.feed(&line);
        let expected = Map {
            width: 10,
            height: 10,
            entries: vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
            ],
        };
        assert_eq!(expected, map);
    }

    #[test]
    fn feed_lines_hor_vert() {
        let mut map = Map::new(10, 10);
        let lines = Sequence::try_from(EXAMPLE_INPUT).unwrap();
        for line in lines.0 {
            map.feed(&line);
        }
        assert_eq!(*MAP, map);
    }
}
