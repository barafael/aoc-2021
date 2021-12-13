#[cfg(test)]
mod tests {
    use crate::day11::{parse_octopi, problem_1::octopus_blinky_party, test::INPUT};

    #[test]
    fn party_with_input_until_sync() {
        let mut party = parse_octopi(INPUT);
        let mut index = 0;
        loop {
            if party.iter().all(|line| line.iter().all(|o| *o == 0)) {
                break;
            }
            let (_, new_party) = octopus_blinky_party(&party);
            party = new_party;
            index += 1;
        }
        assert_eq!(494, index);
    }
}
