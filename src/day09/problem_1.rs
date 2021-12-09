pub fn calculate_risk_factor(height_map: &[Vec<u8>]) -> u64 {
    let max_depth_index = height_map.len() - 1;
    // assumes all lines have equal length and there is at least one line.
    let max_len_index = height_map[0].len() - 1;
    let mut low_points = vec![];
    // inner area.
    for i in 1..max_depth_index {
        for j in 1..max_len_index {
            let value = height_map[i][j];
            if value < height_map[i - 1][j]
                && value < height_map[i][j + 1]
                && value < height_map[i + 1][j]
                && value < height_map[i][j - 1]
            {
                low_points.push(value);
            }
        }
    }
    for j in 1..max_len_index {
        // north row.
        let value = height_map[0][j];
        if value < height_map[0][j + 1] && value < height_map[1][j] && value < height_map[0][j - 1]
        {
            low_points.push(value);
        }
        // south row.
        let value = height_map[max_depth_index][j];
        if value < height_map[max_depth_index][j + 1]
            && value < height_map[max_depth_index - 1][j]
            && value < height_map[max_depth_index][j - 1]
        {
            low_points.push(value);
        }
    }
    for i in 1..max_depth_index {
        // west column.
        let value = height_map[i][0];
        if value < height_map[i - 1][0] && value < height_map[i][1] && value < height_map[i + 1][0]
        {
            low_points.push(value);
        }
        // east column.
        let value = height_map[i][max_len_index];
        if value < height_map[i - 1][max_len_index]
            && value < height_map[i][max_len_index - 1]
            && value < height_map[i + 1][max_len_index]
        {
            low_points.push(value);
        }
    }
    // top left.
    let value = height_map[0][0];
    if value < height_map[1][0] && value < height_map[0][1] {
        low_points.push(value);
    }
    // bottom left.
    let value = height_map[max_depth_index][0];
    if value < height_map[max_depth_index - 1][0] && value < height_map[max_depth_index][1] {
        low_points.push(value);
    }
    // bottom right.
    let value = height_map[max_depth_index][max_len_index];
    if value < height_map[max_depth_index - 1][max_len_index]
        && value < height_map[max_depth_index][max_len_index - 1]
    {
        low_points.push(value);
    }
    // top right.
    let value = height_map[0][max_len_index];
    if value < height_map[1][max_len_index] && value < height_map[0][max_len_index - 1] {
        low_points.push(value);
    }
    low_points.iter().map(|v| *v as u64 + 1).sum()
}

#[cfg(test)]
mod tests {
    use crate::day09::{
        parse_to_vec_vec,
        problem_1::calculate_risk_factor,
        test::{EXAMPLE_INPUT, INPUT},
    };

    #[test]
    fn calculates_sum_of_risk_levels_example_data() {
        let input = parse_to_vec_vec(EXAMPLE_INPUT);
        let sum = calculate_risk_factor(&input);
        assert_eq!(15u64, sum);
    }

    #[test]
    fn calculates_sum_of_risk_levels() {
        let input = parse_to_vec_vec(INPUT);
        let sum = calculate_risk_factor(&input);
        assert_eq!(539u64, sum);
    }
}
