#[cfg(test)]
mod tests {
    use crate::day22::{parse_input, solve, tests::INPUT};

    #[test]
    fn computes_solution() {
        let reboot_steps = parse_input(INPUT, None);
        let r = solve(&reboot_steps);
        assert_eq!(1323862415207825, r);
    }
}
