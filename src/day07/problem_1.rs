pub fn calculate_crab_cost(initial_positions: &[usize]) -> Option<usize> {
    let min = initial_positions.iter().min().unwrap();
    let max = initial_positions.iter().max().unwrap();
    let len = *max - *min;
    let mut sums = vec![0; len];
    let mut least_so_far = usize::MAX;
    for (index, sum) in sums.iter_mut().enumerate() {
        for crab in initial_positions {
            *sum += usize::abs_diff(*crab, index);
            if *sum > least_so_far {
                break;
            }
        }
        if *sum < least_so_far {
            least_so_far = *sum;
        }
    }
    sums.iter().min().copied()
}

#[cfg(test)]
mod tests {
    use crate::day07::{
        parse_to_vec,
        problem_1::calculate_crab_cost,
        test::{EXAMPLE_INPUT, INPUT},
    };

    #[test]
    fn brute_force_on_example_data() {
        let example_data = parse_to_vec(EXAMPLE_INPUT).unwrap();
        let sum = calculate_crab_cost(&example_data);
        assert_eq!(37, sum.unwrap());
    }

    #[test]
    fn computes_solution() {
        let example_data = parse_to_vec(INPUT).unwrap();
        let sum = calculate_crab_cost(&example_data);
        assert_eq!(335330, sum.unwrap());
    }
}
