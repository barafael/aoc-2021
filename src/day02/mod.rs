pub mod parse;
pub mod problem_1;
pub mod problem_2;

#[cfg(test)]
pub const INPUT: &str = include_str!("../../input/day02.txt");

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Position {
    x: i64,
    y: i64,
}
