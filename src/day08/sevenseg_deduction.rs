use std::collections::HashSet;

pub fn deduce_three_and_two_five(
    one: &HashSet<char>,
    three_and_two_five: &[HashSet<char>],
) -> (HashSet<char>, Vec<HashSet<char>>) {
    let (three, two_five): (Vec<_>, Vec<_>) =
        three_and_two_five.iter().cloned().partition(|elem| {
            let union = elem.union(one).copied().collect::<HashSet<_>>();
            *elem == union
        });
    assert_eq!(1, three.len());
    (three[0].clone(), two_five)
}

pub fn deduce_six_and_zero_nine(
    one: &HashSet<char>,
    six_and_zero_nine: &[HashSet<char>],
) -> (HashSet<char>, Vec<HashSet<char>>) {
    let eight = ['a', 'b', 'c', 'd', 'e', 'f', 'g'].into_iter().collect();
    let (six, zero_nine): (Vec<_>, Vec<_>) = six_and_zero_nine.iter().cloned().partition(|elem| {
        let union = elem.union(one).copied().collect::<HashSet<_>>();
        union == eight
    });
    assert_eq!(1, six.len());
    (six[0].clone(), zero_nine)
}

pub fn deduce_zero_and_nine(
    four: &HashSet<char>,
    zero_and_nine: &[HashSet<char>],
) -> (HashSet<char>, HashSet<char>) {
    let (zero, nine): (Vec<_>, Vec<_>) = zero_and_nine.iter().cloned().partition(|elem| {
        let union = elem.union(four).copied().collect::<HashSet<_>>();
        *elem != union
    });
    assert_eq!(1, zero.len());
    assert_eq!(1, nine.len());
    (zero[0].clone(), nine[0].clone())
}

pub fn deduce_two_and_five(
    nine: &HashSet<char>,
    one: &HashSet<char>,
    two_five: &[HashSet<char>],
) -> (HashSet<char>, HashSet<char>) {
    let nine_bar_one = nine.difference(one).copied().collect::<HashSet<_>>();
    let (two, five): (Vec<_>, Vec<_>) = two_five.iter().cloned().partition(|elem| {
        let diff = elem.union(&nine_bar_one).copied().collect::<HashSet<_>>();
        *elem != diff
    });
    assert_eq!(1, two.len());
    assert_eq!(1, five.len());
    (two[0].clone(), five[0].clone())
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::day08::sevenseg_deduction::{deduce_six_and_zero_nine, deduce_three_and_two_five};

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn deduces_three_and_two_five() {
        let two_and_three_five: Vec<HashSet<char>> = vec![
            ['c', 'd', 'f', 'b', 'e'].iter().cloned().collect(),
            ['g', 'c', 'd', 'f', 'a'].iter().cloned().collect(),
            ['f', 'b', 'c', 'a', 'd'].iter().cloned().collect(),
        ];
        let one = ['a', 'b'].iter().cloned().collect();
        let (three, two_five) = deduce_three_and_two_five(&one, &two_and_three_five);
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

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn deduces_six_and_zero_nine() {
        let zero_and_six_nine: Vec<HashSet<char>> = vec![
            ['c', 'a', 'g', 'e', 'd', 'b'].iter().cloned().collect(),
            ['c', 'd', 'f', 'g', 'e', 'b'].iter().cloned().collect(),
            ['c', 'e', 'f', 'a', 'b', 'd'].iter().cloned().collect(),
        ];
        let one = ['a', 'b'].iter().cloned().collect();
        let (six, zero_nine) = deduce_six_and_zero_nine(&one, &zero_and_six_nine);
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

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn deduces_zero_and_nine() {
        let zero_and_nine: Vec<HashSet<char>> = vec![
            ['c', 'a', 'g', 'e', 'd', 'b'].iter().cloned().collect(),
            ['c', 'e', 'f', 'a', 'b', 'd'].iter().cloned().collect(),
        ];
        let four = ['e', 'a', 'f', 'b'].iter().cloned().collect();
        let (zero, nine) = super::deduce_zero_and_nine(&four, &zero_and_nine);
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

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn deduces_two_and_five() {
        let two_and_five: Vec<HashSet<char>> = vec![
            ['c', 'd', 'f', 'b', 'e'].iter().cloned().collect(),
            ['g', 'c', 'd', 'f', 'a'].iter().cloned().collect(),
        ];
        let nine = ['c', 'e', 'f', 'a', 'b', 'd'].iter().cloned().collect();
        let one = ['a', 'b'].iter().cloned().collect();
        let (two, five) = super::deduce_two_and_five(&nine, &one, &two_and_five);
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
