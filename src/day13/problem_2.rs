#[cfg(test)]
mod tests {
    use crate::day13::{parse_manual, test::INPUT};

    #[test]
    fn folds_until_code() {
        let (mut manual, instructions) = parse_manual(INPUT);
        for instruction in &instructions {
            manual.apply_fold(instruction);
        }
        #[allow(non_snake_case)]
        let LGHEGUE = "\
#.....##..#..#.####..##..#..#.####...##
#....#..#.#..#.#....#..#.#..#.#.......#
#....#....####.###..#....#..#.###.....#
#....#.##.#..#.#....#.##.#..#.#.......#
#....#..#.#..#.#....#..#.#..#.#....#..#
####..###.#..#.####..###..##..####..##.
";
        assert_eq!(LGHEGUE, format!("{}", manual));
    }
}
