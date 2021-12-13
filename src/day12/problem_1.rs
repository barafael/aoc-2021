#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::day12::{get_paths, test::INPUT, Caves};

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn counts_paths_in_simple_cave_system() {
        //     start
        //     /   \
        // c--A-----b--d
        //     \   /
        //      end
        let caves = Caves {
            elements: [
                (6, vec![3]),
                (8, vec![4]),
                (18446744073709551615, vec![3, 4]),
                (0, vec![3, 4]),
                (3, vec![0, 6, 4, 18446744073709551615]),
                (4, vec![0, 3, 8, 18446744073709551615]),
            ]
            .iter()
            .cloned()
            .collect(),
            names: [
                (18446744073709551615, "end".into()),
                (6, "c".into()),
                (8, "d".into()),
                (0, "start".into()),
                (4, "b".into()),
                (3, "A".into()),
            ]
            .iter()
            .cloned()
            .collect(),
        };
        assert_eq!(10, get_paths(&caves, false));
    }

    #[test]
    fn computes_solution() {
        let caves = Caves::from_str(INPUT).unwrap();
        assert_eq!(3679, get_paths(&caves, false));
    }
}
