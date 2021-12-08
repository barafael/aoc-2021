use std::collections::HashSet;

pub fn deduce_7seg_cabling(input: &[String], output: &[String]) -> usize {
    assert_eq!(input.len(), 10);
    let input = input
        .iter()
        .map(|input| input.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

    let empty = HashSet::new();
    let mut table = [&empty; 10];

    let eight = filter_by_len(&input, 7);
    assert_eq!(1, eight.len());
    table[8] = &eight[0];

    let seven = filter_by_len(&input, 3);
    assert_eq!(1, seven.len());
    table[7] = &seven[0];

    let four = filter_by_len(&input, 4);
    assert_eq!(1, four.len());
    table[4] = &four[0];

    let one = filter_by_len(&input, 2);
    assert_eq!(1, one.len());
    table[1] = &one[0];

    let zero_six_nine = filter_by_len(&input, 6);
    assert_eq!(3, zero_six_nine.len());

    let two_three_five = filter_by_len(&input, 5);
    assert_eq!(3, two_three_five.len());

    let (six, zero_nine) = deduce_six_and_zero_nine(table[1], zero_six_nine);
    assert_eq!(2, zero_nine.len());
    table[6] = &six;

    let (zero, nine) = deduce_zero_and_nine(table[4], zero_nine);
    table[0] = &zero;
    table[9] = &nine;

    let (three, two_five) = deduce_three_and_two_five(table[1], two_three_five);
    assert_eq!(2, two_five.len());
    table[3] = &three;

    let (two, five) = deduce_two_and_five(table[9], table[1], two_five);
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

fn deduce_three_and_two_five(
    one: &HashSet<char>,
    three_and_two_five: Vec<HashSet<char>>,
) -> (HashSet<char>, Vec<HashSet<char>>) {
    let (three, two_five): (Vec<_>, Vec<_>) =
        three_and_two_five.iter().cloned().partition(|elem| {
            let union = elem.union(one).cloned().collect::<HashSet<_>>();
            *elem == union
        });
    assert_eq!(1, three.len());
    (three[0].clone(), two_five)
}

fn deduce_six_and_zero_nine(
    one: &HashSet<char>,
    six_and_zero_nine: Vec<HashSet<char>>,
) -> (HashSet<char>, Vec<HashSet<char>>) {
    let eight = ['a', 'b', 'c', 'd', 'e', 'f', 'g'].into_iter().collect();
    let (six, zero_nine): (Vec<_>, Vec<_>) = six_and_zero_nine.iter().cloned().partition(|elem| {
        let union = elem.union(one).cloned().collect::<HashSet<_>>();
        union == eight
    });
    assert_eq!(1, six.len());
    (six[0].clone(), zero_nine)
}

fn deduce_zero_and_nine(
    four: &HashSet<char>,
    zero_and_nine: Vec<HashSet<char>>,
) -> (HashSet<char>, HashSet<char>) {
    let (zero, nine): (Vec<_>, Vec<_>) = zero_and_nine.iter().cloned().partition(|elem| {
        let union = elem.union(four).cloned().collect::<HashSet<_>>();
        *elem != union
    });
    assert_eq!(1, zero.len());
    assert_eq!(1, nine.len());
    (zero[0].clone(), nine[0].clone())
}

fn deduce_two_and_five(
    nine: &HashSet<char>,
    one: &HashSet<char>,
    two_five: Vec<HashSet<char>>,
) -> (HashSet<char>, HashSet<char>) {
    let nine_bar_one = nine.difference(one).cloned().collect::<HashSet<_>>();
    let (two, five): (Vec<_>, Vec<_>) = two_five.iter().cloned().partition(|elem| {
        let diff = elem.union(&nine_bar_one).cloned().collect::<HashSet<_>>();
        *elem != diff
    });
    assert_eq!(1, two.len());
    assert_eq!(1, five.len());
    (two[0].clone(), five[0].clone())
}

#[cfg(test)]
mod tests {
    use super::{deduce_7seg_cabling, deduce_three_and_two_five, filter_by_len};
    use crate::day08::{
        parse_to_vec,
        problem_2::deduce_six_and_zero_nine,
        test::{EXAMPLE_INPUT, INPUT},
    };
    use std::collections::HashSet;

    #[test]
    fn computes_from_minimal_input() {
        let input = parse_to_vec(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let sum = deduce_7seg_cabling(&input[0].patterns, &input[0].output);
        assert_eq!(5353, sum);
    }

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

    #[test]
    fn deduces_three_and_two_five() {
        let two_and_three_five: Vec<HashSet<char>> = vec![
            ['c', 'd', 'f', 'b', 'e'].iter().cloned().collect(),
            ['g', 'c', 'd', 'f', 'a'].iter().cloned().collect(),
            ['f', 'b', 'c', 'a', 'd'].iter().cloned().collect(),
        ];
        let one = ['a', 'b'].iter().cloned().collect();
        let (three, two_five) = deduce_three_and_two_five(&one, two_and_three_five);
        assert_eq!(
            ['f', 'b', 'c', 'a', 'd']
                .iter()
                .cloned()
                .collect::<HashSet<_>>(),
            three
        );
        assert_eq!(
            vec![
                ['c', 'd', 'f', 'b', 'e']
                    .iter()
                    .cloned()
                    .collect::<HashSet<_>>(),
                ['g', 'c', 'd', 'f', 'a'].iter().cloned().collect()
            ],
            two_five
        );
    }

    #[test]
    fn deduces_six_and_zero_nine() {
        let zero_and_six_nine: Vec<HashSet<char>> = vec![
            ['c', 'a', 'g', 'e', 'd', 'b'].iter().cloned().collect(),
            ['c', 'd', 'f', 'g', 'e', 'b'].iter().cloned().collect(),
            ['c', 'e', 'f', 'a', 'b', 'd'].iter().cloned().collect(),
        ];
        let one = ['a', 'b'].iter().cloned().collect();
        let (six, zero_nine) = deduce_six_and_zero_nine(&one, zero_and_six_nine);
        assert_eq!(
            ['c', 'd', 'f', 'g', 'e', 'b']
                .iter()
                .cloned()
                .collect::<HashSet<_>>(),
            six
        );
        assert_eq!(
            vec![
                ['c', 'a', 'g', 'e', 'd', 'b']
                    .iter()
                    .cloned()
                    .collect::<HashSet<_>>(),
                ['c', 'e', 'f', 'a', 'b', 'd'].iter().cloned().collect()
            ],
            zero_nine
        );
    }

    #[test]
    fn deduces_zero_and_nine() {
        let zero_and_nine: Vec<HashSet<char>> = vec![
            ['c', 'a', 'g', 'e', 'd', 'b'].iter().cloned().collect(),
            ['c', 'e', 'f', 'a', 'b', 'd'].iter().cloned().collect(),
        ];
        let four = ['e', 'a', 'f', 'b'].iter().cloned().collect();
        let (zero, nine) = super::deduce_zero_and_nine(&four, zero_and_nine);
        assert_eq!(
            ['c', 'a', 'g', 'e', 'd', 'b']
                .iter()
                .cloned()
                .collect::<HashSet<_>>(),
            zero
        );
        assert_eq!(
            ['c', 'e', 'f', 'a', 'b', 'd']
                .iter()
                .cloned()
                .collect::<HashSet<_>>(),
            nine
        );
    }

    #[test]
    fn deduces_two_and_five() {
        let two_and_five: Vec<HashSet<char>> = vec![
            ['c', 'd', 'f', 'b', 'e'].iter().cloned().collect(),
            ['g', 'c', 'd', 'f', 'a'].iter().cloned().collect(),
        ];
        let nine = ['c', 'e', 'f', 'a', 'b', 'd'].iter().cloned().collect();
        let one = ['a', 'b'].iter().cloned().collect();
        let (two, five) = super::deduce_two_and_five(&nine, &one, two_and_five);
        assert_eq!(
            ['g', 'c', 'd', 'f', 'a']
                .iter()
                .cloned()
                .collect::<HashSet<_>>(),
            two
        );
        assert_eq!(
            ['c', 'd', 'f', 'b', 'e']
                .iter()
                .cloned()
                .collect::<HashSet<_>>(),
            five
        );
    }
}
