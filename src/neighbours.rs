/// Assumes all rows have equal length.
pub fn direct_neighbours_of(input: &[Vec<u8>], seed: (usize, usize)) -> Vec<(usize, usize)> {
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

/// Assumes all rows have equal length.
pub fn diagonal_neighbours_of(input: &[Vec<u8>], seed: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    // top left.
    if seed.0 != 0 && seed.1 != 0 {
        neighbours.push((seed.0 - 1, seed.1 - 1));
    }
    // top right.
    if seed.0 != 0 && seed.1 != input[0].len() - 1 {
        neighbours.push((seed.0 - 1, seed.1 + 1));
    }
    // bottom left.
    if seed.0 != input.len() - 1 && seed.1 != 0 {
        neighbours.push((seed.0 + 1, seed.1 - 1));
    }
    // bottom right.
    if seed.0 != input.len() - 1 && seed.1 != input[0].len() - 1 {
        neighbours.push((seed.0 + 1, seed.1 + 1));
    }
    neighbours
}

#[cfg(feature = "non_solution_test")]
#[cfg(test)]
mod test {
    use crate::neighbours::{diagonal_neighbours_of, direct_neighbours_of};

    #[test]
    fn direct_neighbours() {
        let input = crate::day09::parse_to_vec_vec(include_str!("../input/day09.txt"));
        let neighbours = direct_neighbours_of(&input, (0, 0));
        assert_eq!(vec![(0, 1), (1, 0)], neighbours);
        let neighbours = direct_neighbours_of(&input, (2, 4));
        assert_eq!(vec![(1, 4), (2, 5), (3, 4), (2, 3)], neighbours);
    }

    #[test]
    fn diagonal_neighbours() {
        let input = crate::day11::parse_octopi(include_str!("../input/day11.txt"));
        let neighbours = diagonal_neighbours_of(&input, (0, 0));
        assert_eq!(vec![(1, 1)], neighbours);
        let neighbours = diagonal_neighbours_of(&input, (2, 4));
        assert_eq!(vec![(1, 3), (1, 5), (3, 3), (3, 5)], neighbours);
    }
}
