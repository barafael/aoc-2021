pub mod map;
pub mod parse;
pub mod problem_1;
pub mod problem_2;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Line {
    start: Point,
    end: Point,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Sequence(Vec<Line>);

impl Line {
    pub const fn is_diagonal(&self) -> Option<usize> {
        let x_diff = usize::abs_diff(self.start.x, self.end.x);
        let y_diff = usize::abs_diff(self.start.y, self.end.y);
        if x_diff == y_diff {
            Some(x_diff)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    pub const INPUT: &str = include_str!("../../input/day05.txt");

    #[cfg(feature = "non_solution_test")]
    pub const EXAMPLE_INPUT: &str = r##"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"##;

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn point_partial_eq() {
        use crate::day05::Point;
        let a = Point { x: 1, y: 4 };
        let b = Point { x: 5, y: 9 };
        assert!(a <= b);
    }
}
