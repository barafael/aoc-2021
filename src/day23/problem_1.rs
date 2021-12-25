#[cfg(test)]
mod tests {
    use crate::day23::{parse_maze, run_game, tests::INPUT};

    #[test]
    pub fn computes_solution() {
        let maze = parse_maze(INPUT);
        let (_, cost) = run_game(&maze);
        assert_eq!(cost, 11536);
    }
}
