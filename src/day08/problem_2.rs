use std::collections::HashSet;

use crate::day08::sevenseg_deduction::{
    deduce_six_and_zero_nine, deduce_three_and_two_five, deduce_two_and_five, deduce_zero_and_nine,
};

pub fn deduce_7seg_cabling(input: &[HashSet<char>], output: &[String]) -> usize {
    assert_eq!(input.len(), 10);

    let empty = HashSet::new();
    let mut table = [&empty; 10];

    let eight = filter_by_len(input, 7);
    assert_eq!(1, eight.len());
    table[8] = &eight[0];

    let seven = filter_by_len(input, 3);
    assert_eq!(1, seven.len());
    table[7] = &seven[0];

    let four = filter_by_len(input, 4);
    assert_eq!(1, four.len());
    table[4] = &four[0];

    let one = filter_by_len(input, 2);
    assert_eq!(1, one.len());
    table[1] = &one[0];

    let zero_six_nine = filter_by_len(input, 6);
    assert_eq!(3, zero_six_nine.len());

    let two_three_five = filter_by_len(input, 5);
    assert_eq!(3, two_three_five.len());

    let (six, zero_nine) = deduce_six_and_zero_nine(table[1], &zero_six_nine);
    assert_eq!(2, zero_nine.len());
    table[6] = &six;

    let (zero, nine) = deduce_zero_and_nine(table[4], &zero_nine);
    table[0] = &zero;
    table[9] = &nine;

    let (three, two_five) = deduce_three_and_two_five(table[1], &two_three_five);
    assert_eq!(2, two_five.len());
    table[3] = &three;

    let (two, five) = deduce_two_and_five(table[9], table[1], &two_five);
    table[2] = &two;
    table[5] = &five;

    let mut digit = vec![];
    for s in output {
        let s: HashSet<char> = s.chars().collect::<HashSet<_>>();
        for (index, c) in table.iter().enumerate() {
            if *c == &s {
                digit.push(index);
            }
        }
    }
    digit[0] * 1000 + digit[1] * 100 + digit[2] * 10 + digit[3]
}

fn filter_by_len(input: &[HashSet<char>], len: usize) -> Vec<HashSet<char>> {
    input
        .iter()
        .filter(|char| char.len() == len)
        .cloned()
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::{deduce_7seg_cabling, filter_by_len};
    use crate::day08::{
        parse_to_vec,
        test::{EXAMPLE_INPUT, INPUT},
    };
    use std::collections::HashSet;

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn computes_from_minimal_input() {
        let input = parse_to_vec(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let sum = deduce_7seg_cabling(&input[0].patterns, &input[0].output);
        assert_eq!(5353, sum);
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn computes_from_example_input() {
        let input = parse_to_vec(EXAMPLE_INPUT);
        let mut sum = 0;
        for entry in input {
            sum += deduce_7seg_cabling(&entry.patterns, &entry.output);
        }
        assert_eq!(61229, sum);
    }

    #[test]
    fn computes_result() {
        let input = parse_to_vec(INPUT);
        let mut sum = 0;
        for entry in input {
            sum += deduce_7seg_cabling(&entry.patterns, &entry.output);
        }
        assert_eq!(994266, sum);
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn filters_by_len() {
        let sets = ["ab", "abc", "def", "fga", "ez"]
            .iter()
            .map(|chars| chars.chars().collect::<HashSet<_>>())
            .collect::<Vec<HashSet<_>>>();

        let filtered = filter_by_len(&sets, 3);
        assert_eq!(3, filtered.len());
        assert!(filtered.iter().all(|s| s.len() == 3));
    }
}
