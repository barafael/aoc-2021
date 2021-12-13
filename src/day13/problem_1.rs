#[cfg(test)]
mod tests {
    use crate::day13::{parse_manual, test::INPUT};

    #[test]
    fn folds_once() {
        let (mut manual, instructions) = parse_manual(INPUT);
        manual.apply_fold(&instructions[0]);
        assert_eq!(807, manual.coordinates.len());
    }
}
