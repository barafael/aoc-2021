use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Add};

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum Number {
    Literal(u64),
    Pair(Box<Self>, Box<Self>),
}

impl Number {
    pub fn pair(lhs: Self, rhs: Self) -> Self {
        Self::Pair(Box::new(lhs), Box::new(rhs))
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(l) => write!(f, "{}", l),
            Self::Pair(a, b) => write!(f, "[{},{}]", a, b),
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = Self::pair(self, rhs);
        loop {
            let mut stable = true;
            sum.explode_helper(&mut stable, 0);
            if !stable {
                continue;
            }
            sum.split(&mut stable);
            if stable {
                break;
            }
        }
        sum
    }
}

impl Number {
    fn explode_helper(
        &mut self,
        stable: &mut bool,
        lvl: u8,
    ) -> (Option<ExplodeCarry>, Option<ExplodeCarry>) {
        if !*stable {
            return (None, None);
        }

        if matches!(self, Self::Literal(_)) {
            return (None, None);
        }

        if lvl == 4 {
            if let Self::Pair(box Number::Literal(l), box Number::Literal(r)) = self {
                let l_exp = ExplodeCarry::Lhs(*l);
                let r_exp = ExplodeCarry::Rhs(*r);

                *self = Self::Literal(0);
                *stable = false;

                return (Some(l_exp), Some(r_exp));
            }
            panic!("Expected a pair of literals");
        }

        if lvl < 4 {
            if let Self::Pair(l, r) = self {
                if let (Some(l_ec), Some(r_ec)) = l.explode_helper(stable, lvl + 1) {
                    let r_ec = r.carry(&r_ec);
                    return (Some(l_ec), Some(r_ec));
                } else if let (Some(l_exp), Some(r_exp)) = r.explode_helper(stable, lvl + 1) {
                    let l_exp = l.carry(&l_exp);

                    return (Some(l_exp), Some(r_exp));
                }
            }
        }
        (None, None)
    }

    fn carry(&mut self, carried: &ExplodeCarry) -> ExplodeCarry {
        match carried {
            ExplodeCarry::Consumed => ExplodeCarry::Consumed,
            ExplodeCarry::Lhs(c) => match self {
                Number::Literal(n) => {
                    *n += c;

                    ExplodeCarry::Consumed
                }
                Number::Pair(_, r) => r.carry(carried),
            },
            ExplodeCarry::Rhs(c) => match self {
                Number::Literal(n) => {
                    *n += c;

                    ExplodeCarry::Consumed
                }
                Number::Pair(l, _) => l.carry(carried),
            },
        }
    }

    fn split(&mut self, stable: &mut bool) {
        if !*stable {
            return;
        }

        match self {
            Self::Literal(n) => {
                if *n >= 10 {
                    let l = *n / 2;
                    let r = ((*n as f32) / 2f32).ceil() as u64;

                    *self = Self::pair(Self::Literal(l), Self::Literal(r));
                    *stable = false;
                }
            }
            Self::Pair(l, r) => {
                l.split(stable);
                r.split(stable);
            }
        }
    }

    pub fn magnitude(&self) -> usize {
        match self {
            Self::Literal(n) => *n as usize,
            Self::Pair(box Number::Literal(l), box Number::Literal(r)) => {
                3 * (*l as usize) + 2 * (*r as usize)
            }
            Self::Pair(l, r) => {
                let l = l.magnitude();
                let r = r.magnitude();

                3 * (l as usize) + 2 * (r as usize)
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ExplodeCarry {
    Consumed,
    Lhs(u64),
    Rhs(u64),
}

#[cfg(test)]
mod test {
    use crate::day18::snailfish_math::number::Number;

    #[test]
    fn number_displays() {
        let number = Number::pair(
            Number::Literal(9),
            Number::pair(Number::Literal(8), Number::Literal(7)),
        );
        assert_eq!("[9,[8,7]]", format!("{}", number));
    }

    #[test]
    fn nested_number_displays() {
        let number = Number::pair(
            Number::pair(
                Number::pair(Number::Literal(4), Number::Literal(3)),
                Number::Literal(5),
            ),
            Number::pair(Number::Literal(8), Number::Literal(7)),
        );
        assert_eq!("[[[4,3],5],[8,7]]", format!("{}", number));
    }

    #[test]
    fn pairing() {
        let lhs = Number::pair(
            Number::Literal(1),
            Number::pair(Number::Literal(12), Number::Literal(10)),
        );
        let rhs = Number::Literal(13);
        let result = Number::pair(lhs, rhs);
        assert_eq!(
            Number::pair(
                Number::pair(
                    Number::Literal(1),
                    Number::pair(Number::Literal(12), Number::Literal(10),)
                ),
                Number::Literal(13),
            ),
            result
        );
    }

    #[test]
    fn explodes() {
        let number = Number::pair(Number::Literal(2), Number::Literal(4));
        let number = Number::pair(number.clone(), number.clone());
        let number = Number::pair(number.clone(), number.clone());
        let number = Number::pair(number.clone(), number.clone());
        let mut number = Number::pair(number.clone(), number.clone());
        let mut stable = true;
        println!("{}", number);
        number.explode_helper(&mut stable, 0);
        println!("{}", number);
    }
}
