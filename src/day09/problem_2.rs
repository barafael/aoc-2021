use crate::neighbours::direct_neighbours_of;
use std::collections::{HashSet, VecDeque};

pub fn expand_region(input: &[Vec<u8>], seed: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::<(usize, usize)>::new();
    to_visit.push_back(seed);
    while let Some(v) = to_visit.pop_front() {
        let neighbours = direct_neighbours_of(input, v);
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

#[cfg(test)]
mod tests {
    use crate::day09::{find_low_points, parse_to_vec_vec, test::INPUT};
    use itertools::Itertools;

    #[cfg(feature = "non_solution_test")]
    use crate::day09::test::EXAMPLE_INPUT;
    #[cfg(feature = "non_solution_test")]
    use std::collections::HashSet;

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn expand_region() {
        let input = parse_to_vec_vec(EXAMPLE_INPUT);
        let region = super::expand_region(&input, (0, 1));
        assert_eq!(
            [(0, 1), (0, 0), (1, 0)].into_iter().collect::<HashSet<_>>(),
            region
        );
    }

    #[cfg(feature = "non_solution_test")]
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
