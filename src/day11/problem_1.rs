use itertools::Itertools;

use crate::neighbours::{diagonal_neighbours_of, direct_neighbours_of};

pub fn prettyprint_party(party: Vec<Vec<u8>>) {
    println!(
        "{:?}",
        party.iter().map(|l| l.iter().format("")).format("\n")
    );
}

pub fn octopus_blinky_party(input: Vec<Vec<u8>>) -> (usize, Vec<Vec<u8>>) {
    let mut step_1 = input
        .iter()
        .map(|l| l.iter().map(|o| o + 1).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;

    loop {
        let partying: usize = step_1
            .iter()
            .map(|l| l.iter().filter(|o| *o > &9).count())
            .sum();
        if partying == 0 {
            break;
        }
        let mut mask = vec![vec![0; 10]; 10];
        for (i, line) in step_1.iter_mut().enumerate() {
            for (j, elem) in line.iter_mut().enumerate() {
                if *elem > 9 {
                    *elem = 0;
                    count += 1;
                    let direct_neighbours = direct_neighbours_of(&input, (i, j));
                    let diagonal_neighbours = diagonal_neighbours_of(&input, (i, j));
                    for neighbours in direct_neighbours.iter().chain(&diagonal_neighbours) {
                        mask[neighbours.0][neighbours.1] += 1;
                    }
                }
            }
        }
        for (i, line) in step_1.iter_mut().enumerate() {
            for (j, o) in line.iter_mut().enumerate() {
                if *o != 0 {
                    *o += mask[i][j];
                }
            }
        }
    }
    (count, step_1)
}

#[cfg(test)]
mod tests {
    use super::octopus_blinky_party;
    use crate::day11::{parse_octopi, test::INPUT};

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn party_with_example_data() {
        let initial = "11111
19991
19191
19991
11111";
        let party = parse_octopi(initial);
        let party = octopus_blinky_party(party);
        assert_eq!(
            (
                9,
                vec![
                    vec![3, 4, 5, 4, 3],
                    vec![4, 0, 0, 0, 4],
                    vec![5, 0, 0, 0, 5],
                    vec![4, 0, 0, 0, 4],
                    vec![3, 4, 5, 4, 3]
                ],
            ),
            party
        );
        let party = octopus_blinky_party(party.1);
        assert_eq!(
            (
                0,
                vec![
                    vec![4, 5, 6, 5, 4],
                    vec![5, 1, 1, 1, 5],
                    vec![6, 1, 1, 1, 6],
                    vec![5, 1, 1, 1, 5],
                    vec![4, 5, 6, 5, 4],
                ],
            ),
            party
        );
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn party_with_example_input_10_steps() {
        let mut party = parse_octopi(
            "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
        );
        let mut sum = 0;
        for _i in 0..10 {
            let (flashes, new_party) = octopus_blinky_party(party);
            party = new_party;
            sum += flashes;
        }
        assert_eq!(204, sum);
        assert_eq!(
            parse_octopi(
                "0481112976
0031112009
0041112504
0081111406
0099111306
0093511233
0442361130
5532252350
0532250600
0032240000"
            ),
            party
        );
    }

    #[test]
    fn party_with_input_100_steps() {
        let mut party = parse_octopi(INPUT);
        let mut sum = 0;
        for _i in 0..100 {
            let (flashes, new_party) = octopus_blinky_party(party);
            party = new_party;
            sum += flashes;
        }
        assert_eq!(1773, sum);
    }
}
