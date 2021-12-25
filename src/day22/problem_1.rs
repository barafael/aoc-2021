#[cfg(test)]
mod tests {
    use crate::day22::{parse_input, solve, tests::INPUT};

    #[test]
    fn computes_solution() {
        let reboot_steps = parse_input(INPUT, Some(&(-50..=50)));
        let r = solve(&reboot_steps);
        assert_eq!(615869, r);
    }
}
