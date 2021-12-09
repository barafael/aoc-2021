use std::collections::{HashSet, VecDeque};

pub fn expand_region(input: &[Vec<u8>], seed: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::<(usize, usize)>::new();
    to_visit.push_back(seed);
    while let Some(v) = to_visit.pop_front() {
        let neighbours = get_neighbours(input, v);
        let neighbours = neighbours
            .iter()
            .filter(|(i, j)| input[*i][*j] != 9)
            .filter(|c| !to_visit.contains(c))
            .filter(|c| !visited.contains(*c))
            .copied()
            .collect::<Vec<(usize, usize)>>();
        neighbours.iter().for_each(|e| to_visit.push_back(*e));
        visited.insert(v);
    }
    visited
}

fn get_neighbours(input: &[Vec<u8>], seed: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    // top.
    if seed.0 != 0 {
        let i = seed.0 - 1;
        neighbours.push((i, seed.1));
    }
    // right.
    if seed.1 != input[0].len() - 1 {
        let j = seed.1 + 1;
        neighbours.push((seed.0, j));
    }
    // bottom.
    if seed.0 != input.len() - 1 {
        let i = seed.0 + 1;
        neighbours.push((i, seed.1));
    }
    // right.
    if seed.1 != 0 {
        let j = seed.1 - 1;
        neighbours.push((seed.0, j));
    }
    neighbours
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::day09::{
        find_low_points, parse_to_vec_vec,
        problem_2::get_neighbours,
        test::{EXAMPLE_INPUT, INPUT},
    };
    use std::collections::HashSet;

    #[test]
    fn gets_neighbours() {
        let input = parse_to_vec_vec(EXAMPLE_INPUT);
        let neighbours = get_neighbours(&input, (0, 0));
        assert_eq!(2, neighbours.len());
        assert_eq!(vec![(0, 1), (1, 0)], neighbours);

        let neighbours = get_neighbours(&input, (2, 4));
        assert_eq!(vec![(1, 4), (2, 5), (3, 4), (2, 3)], neighbours);
    }

    #[test]
    fn expand_region() {
        let input = parse_to_vec_vec(EXAMPLE_INPUT);
        let region = super::expand_region(&input, (0, 1));
        assert_eq!(
            [(0, 1), (0, 0), (1, 0)].into_iter().collect::<HashSet<_>>(),
            region
        );
    }

    #[test]
    fn computes_example_result() {
        let input = parse_to_vec_vec(EXAMPLE_INPUT);
        let low_points = find_low_points(&input);
        let result = low_points
            .iter()
            .copied()
            .map(|point| super::expand_region(&input, point))
            .sorted_by(|one, other| other.len().cmp(&one.len()))
            .take(3)
            .map(|s| s.len())
            .fold(1, |acc, new| acc * new);
        assert_eq!(1134, result);
    }

    #[test]
    fn computes_result() {
        let input = parse_to_vec_vec(INPUT);
        let low_points = find_low_points(&input);
        let result = low_points
            .iter()
            .copied()
            .map(|point| super::expand_region(&input, point))
            .sorted_by(|one, other| other.len().cmp(&one.len()))
            .take(3)
            .map(|s| s.len())
            .fold(1, |acc, new| acc * new);
        assert_eq!(736920, result);
    }
}
