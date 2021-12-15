pub fn extend_5x5_tiled(grid: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut result = vec![vec![0; 5 * grid[0].len()]; 5 * grid.len()];
    for i in 0..5 {
        for j in 0..5 {
            let offset = i + j;
            let grid = grid
                .iter()
                .map(|l| {
                    l.iter()
                        .map(|elem| {
                            let new = elem + offset;
                            if new > 9 {
                                new - 9
                            } else {
                                new
                            }
                        })
                        .collect()
                })
                .collect();
            copy_grid_onto(&grid, &mut result, (i * grid[0].len(), j * grid.len()));
        }
    }
    result
}

fn copy_grid_onto(grid: &Vec<Vec<usize>>, onto: &mut Vec<Vec<usize>>, coords: (usize, usize)) {
    for (i, line) in grid.iter().enumerate() {
        for (j, elem) in line.iter().enumerate() {
            onto[i + coords.0][j + coords.1] = *elem;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{copy_grid_onto, extend_5x5_tiled};
    use crate::day15::{
        graph_from_grid, parse_grid,
        test::{EXAMPLE_INPUT, INPUT},
    };
    use itertools::Itertools;

    #[test]
    fn copies_grid_onto() {
        let mut onto = vec![vec![1; 10]; 10];
        let grid = parse_grid(
            "\
1122
3344
5566
",
        );
        copy_grid_onto(&grid, &mut onto, (7, 5));
        let expected = "\
1111111111
1111111111
1111111111
1111111111
1111111111
1111111111
1111111111
1111111221
1111133441
1111155661
";
        let result = onto
            .iter()
            .map(|line| format!("{}\n", line.iter().format("")))
            .collect::<String>();
        assert_eq!(expected, result);
    }

    #[test]
    fn computes_example_solution() {
        let grid = parse_grid(EXAMPLE_INPUT);
        let large_grid = extend_5x5_tiled(grid);
        let string = large_grid
            .iter()
            .map(|line| format!("{}\n", line.iter().format("")))
            .collect::<String>();
        println!("{}", string);
        let graph = graph_from_grid(large_grid);
        let start = graph.nodes().find(|node| *node == (0, 0)).unwrap();
        let goal = graph.nodes().find(|node| *node == (49, 49)).unwrap();
        let result = petgraph::algo::dijkstra(&graph, start, Some(goal), |s| *s.2);
        assert_eq!(315, result[&(49, 49)]);
    }

    #[test]
    fn computes_solution() {
        let grid = parse_grid(INPUT);
        let large_grid = extend_5x5_tiled(grid);
        let graph = graph_from_grid(large_grid);
        let start = graph.nodes().find(|node| *node == (0, 0)).unwrap();
        let goal = graph.nodes().find(|node| *node == (499, 499)).unwrap();
        let result = petgraph::algo::dijkstra(&graph, start, Some(goal), |s| *s.2);
        assert_eq!(2927, result[&(499, 499)]);
    }
}
