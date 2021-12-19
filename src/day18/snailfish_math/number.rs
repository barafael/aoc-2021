use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum Number {
    Literal(u64),
    Pair(Box<Number>, Box<Number>),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Literal(l) => write!(f, "{}", l),
            Number::Pair(a, b) => write!(f, "[{},{}]", a, b),
        }
    }
}
