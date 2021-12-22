use super::player::Player;
use std::{collections::HashMap, hash::Hash, str::FromStr};

#[derive(Clone, Hash, Debug, Default, PartialEq, Eq)]
pub struct Situation {
    player: Vec<Player>,
    next_player: usize,
}

impl FromStr for Situation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = &mut s.lines();
        let player_1 = Player::from_str(lines.next().unwrap()).unwrap();
        let player_2 = Player::from_str(lines.next().unwrap()).unwrap();
        Ok(Self {
            player: vec![player_1, player_2],
            next_player: 0,
        })
    }
}

impl Situation {
    pub fn new(pos1: u16, pos2: u16) -> Self {
        Self {
            player: vec![
                Player {
                    position: pos1,
                    score: 0,
                },
                Player {
                    position: pos2,
                    score: 0,
                },
            ],
            next_player: 0,
        }
    }

    pub fn get_wins(self, visited: &mut HashMap<Self, [u64; 2]>) -> [u64; 2] {
        if visited.contains_key(&self) {
            return *visited.get(&self).unwrap();
        }
        let mut res = [0, 0];
        for a in 1..=3 {
            for b in 1..=3 {
                for c in 1..=3 {
                    let mut n = self.clone();
                    n.player[n.next_player].position =
                        (n.player[n.next_player].position + a + b + c - 1) % 10 + 1;
                    n.player[n.next_player].score += n.player[n.next_player].position;
                    if n.player[n.next_player].score >= 21 {
                        res[n.next_player] += 1;
                    } else {
                        n.next_player = 1 - n.next_player;
                        let r = n.get_wins(visited);
                        res[0] += r[0];
                        res[1] += r[1];
                    }
                }
            }
        }
        visited.insert(self, res);
        res
    }
}
