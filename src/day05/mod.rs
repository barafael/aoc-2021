pub mod map;
pub mod parse;
pub mod problem_1;
pub mod problem_2;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

#[cfg(test)]
mod test {
    pub const INPUT: &str = include_str!("../../input/day05.txt");
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
}
