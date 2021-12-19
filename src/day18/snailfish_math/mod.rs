pub mod number;
pub mod parser;

#[cfg(test)]
mod test {
    use crate::day18::snailfish_math::number::Number;

    #[test]
    fn number_displays() {
        let number = Number::Pair(
            Box::new(Number::Literal(9)),
            Box::new(Number::Pair(
                Box::new(Number::Literal(8)),
                Box::new(Number::Literal(7)),
            )),
        );
        assert_eq!("[9,[8,7]]", format!("{}", number));
    }

    #[test]
    fn nested_number_displays() {
        let number = Number::Pair(
            Box::new(Number::Pair(
                Box::new(Number::Pair(
                    Box::new(Number::Literal(4)),
                    Box::new(Number::Literal(3)),
                )),
                Box::new(Number::Literal(5)),
            )),
            Box::new(Number::Pair(
                Box::new(Number::Literal(8)),
                Box::new(Number::Literal(7)),
            )),
        );
        assert_eq!("[[[4,3],5],[8,7]]", format!("{}", number));
    }
}
