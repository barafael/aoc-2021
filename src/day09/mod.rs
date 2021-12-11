pub mod problem_1;
pub mod problem_2;

pub fn parse_to_vec_vec(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).map(|n| n as u8))
                .collect::<Option<Vec<_>>>()
                .unwrap()
        })
        .collect::<Vec<_>>()
}

pub fn find_low_points(height_map: &[Vec<u8>]) -> Vec<(usize, usize)> {
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
                low_points.push((i, j));
            }
        }
    }
    for j in 1..max_len_index {
        // north row.
        let value = height_map[0][j];
        if value < height_map[0][j + 1] && value < height_map[1][j] && value < height_map[0][j - 1]
        {
            low_points.push((0, j));
        }
        // south row.
        let value = height_map[max_depth_index][j];
        if value < height_map[max_depth_index][j + 1]
            && value < height_map[max_depth_index - 1][j]
            && value < height_map[max_depth_index][j - 1]
        {
            low_points.push((max_depth_index, j));
        }
    }
    for i in 1..max_depth_index {
        // west column.
        let value = height_map[i][0];
        if value < height_map[i - 1][0] && value < height_map[i][1] && value < height_map[i + 1][0]
        {
            low_points.push((i, 0));
        }
        // east column.
        let value = height_map[i][max_len_index];
        if value < height_map[i - 1][max_len_index]
            && value < height_map[i][max_len_index - 1]
            && value < height_map[i + 1][max_len_index]
        {
            low_points.push((i, max_len_index));
        }
    }
    // top left.
    let value = height_map[0][0];
    if value < height_map[1][0] && value < height_map[0][1] {
        low_points.push((0, 0));
    }
    // bottom left.
    let value = height_map[max_depth_index][0];
    if value < height_map[max_depth_index - 1][0] && value < height_map[max_depth_index][1] {
        low_points.push((max_depth_index, 0));
    }
    // bottom right.
    let value = height_map[max_depth_index][max_len_index];
    if value < height_map[max_depth_index - 1][max_len_index]
        && value < height_map[max_depth_index][max_len_index - 1]
    {
        low_points.push((max_depth_index, max_len_index));
    }
    // top right.
    let value = height_map[0][max_len_index];
    if value < height_map[1][max_len_index] && value < height_map[0][max_len_index - 1] {
        low_points.push((0, max_len_index));
    }
    low_points
}

#[cfg(test)]
mod test {
    use super::parse_to_vec_vec;

    pub const INPUT: &str = include_str!("../../input/day09.txt");
    pub const EXAMPLE_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn parses_input_ok() {
        let _input = parse_to_vec_vec(INPUT);
    }
}
