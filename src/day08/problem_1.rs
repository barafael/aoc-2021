use super::Entry;

pub fn count_1_4_7_8(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(|entry| {
            entry
                .output
                .iter()
                .filter(|output| {
                    output.len() == 2 || output.len() == 3 || output.len() == 4 || output.len() == 7
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day08::{parse_to_vec, problem_1::count_1_4_7_8, test::INPUT};

    #[cfg(feature = "non_solution_test")]
    use crate::day08::test::EXAMPLE_INPUT;

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn computes_26_instances_in_example_input() {
        let entries = parse_to_vec(EXAMPLE_INPUT);
        let sum = count_1_4_7_8(&entries);
        assert_eq!(26, sum);
    }

    #[test]
    fn computes_sum_of_1_4_7_8() {
        let entries = parse_to_vec(INPUT);
        let sum = count_1_4_7_8(&entries);
        assert_eq!(543, sum);
    }
}
