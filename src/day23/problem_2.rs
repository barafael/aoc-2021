#[cfg(test)]
mod tests {
    use crate::day23::{extend, parse_maze, run_game, tests::INPUT};

    #[test]
    pub fn computes_solution() {
        let maze = parse_maze(INPUT);
        let maze = extend(&maze);
        let (_, cost) = run_game(&maze);
        assert_eq!(cost, 55136);
    }
}
