pub fn calculate_crab_cost(initial_positions: &[usize]) -> Option<usize> {
    let min = initial_positions.iter().min().unwrap();
    let max = initial_positions.iter().max().unwrap();
    let len = *max - *min;
    let mut sums = vec![0; len];
    let mut least_so_far = usize::MAX;
    for (index, sum) in sums.iter_mut().enumerate() {
        for crab in initial_positions {
            let distance = usize::abs_diff(*crab, index);
            let cost = (distance * (distance + 1)) / 2;
            *sum += cost;
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
        problem_2::calculate_crab_cost,
        test::{EXAMPLE_INPUT, INPUT},
    };

    #[test]
    fn calculates_cost_for_example_data() {
        let data = parse_to_vec(EXAMPLE_INPUT).unwrap();
        let sum = calculate_crab_cost(&data);
        assert_eq!(168, sum.unwrap());
    }

    #[test]
    fn computes_solution() {
        let data = parse_to_vec(INPUT).unwrap();
        let sum = calculate_crab_cost(&data);
        assert_eq!(92439766, sum.unwrap());
    }
}
