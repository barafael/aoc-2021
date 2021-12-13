#[cfg(test)]
mod tests {
    use crate::day13::{test::INPUT, UserManual};
    use std::str::FromStr;

    #[test]
    fn folds_once() {
        let mut manual = UserManual::from_str(INPUT).unwrap();
        manual.apply_fold();
        assert_eq!(807, manual.coordinates.len());
    }
}
