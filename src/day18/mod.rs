pub mod problem_1;
pub mod problem_2;
pub mod snailfish_math;

#[cfg(test)]
mod test {
    pub const INPUT: &str = include_str!("../../input/day18.txt");

    pub const EXAMPLE: &str = "\
[1,2]
[[1,2],3]
[9,[8,7]]
[[1,9],[8,5]]
[[[[1,2],[3,4]],[[5,6],[7,8]]],9]
[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]\n";
}
