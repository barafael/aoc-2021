use crate::neighbours;

pub mod problem_2;

pub fn parse_grid(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn graph_from_grid(
    grid: &[Vec<usize>],
) -> petgraph::graphmap::DiGraphMap<(usize, usize), usize> {
    let mut matrix_graph = petgraph::graphmap::DiGraphMap::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let neighbours = neighbours::direct_neighbours_of(grid, (i, j));
            let node = matrix_graph.add_node((i, j));
            for neighbour in neighbours {
                let n_idx = matrix_graph.add_node(neighbour);
                matrix_graph.add_edge(node, n_idx, grid[neighbour.0][neighbour.1]);
            }
        }
    }
    matrix_graph
}

#[cfg(test)]
mod test {
    use super::graph_from_grid;
    use crate::day15::parse_grid;
    use petgraph::dot::Dot;

    pub const INPUT: &str = include_str!("../../input/day15.txt");

    pub const EXAMPLE_INPUT: &str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn parses_graph() {
        let grid = parse_grid(EXAMPLE_INPUT);
        let graph = graph_from_grid(&grid);
        assert_eq!(
            include_str!("example_graph.dot"),
            format!("{:?}", Dot::with_config(&graph, &[]))
        );
    }

    #[test]
    fn finds_shortest_path_in_example() {
        let grid = parse_grid(EXAMPLE_INPUT);
        let graph = graph_from_grid(&grid);
        let start = graph.nodes().find(|node| *node == (0, 0)).unwrap();
        let goal = graph.nodes().find(|node| *node == (9, 9)).unwrap();
        let result = petgraph::algo::dijkstra(&graph, start, Some(goal), |s| *s.2);
        assert_eq!(40, result[&(9, 9)]);
    }

    #[test]
    fn finds_shortest_path() {
        let grid = parse_grid(INPUT);
        let graph = graph_from_grid(&grid);
        let start = graph.nodes().find(|node| *node == (0, 0)).unwrap();
        let goal = graph.nodes().find(|node| *node == (99, 99)).unwrap();
        let result = petgraph::algo::dijkstra(&graph, start, Some(goal), |s| *s.2);
        assert_eq!(583, result[&(99, 99)]);
    }
}
