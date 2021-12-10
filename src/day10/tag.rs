#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    OpenRoundParenthesis,
    CloseParenthesis,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenCurlyBrace,
    CloseCurlyBrace,
    CloseAngleBracket,
    OpenAngleBracket,
}

impl Tag {
    pub const fn is_opening(&self) -> bool {
        use Tag::*;
        match self {
            OpenRoundParenthesis | OpenSquareBracket | OpenCurlyBrace | OpenAngleBracket => true,
            CloseParenthesis | CloseSquareBracket | CloseCurlyBrace | CloseAngleBracket => false,
        }
    }

    pub const fn pair(&self) -> Self {
        use self::Tag::*;
        match self {
            OpenRoundParenthesis => CloseParenthesis,
            CloseParenthesis => OpenRoundParenthesis,
            OpenSquareBracket => CloseSquareBracket,
            CloseSquareBracket => OpenSquareBracket,
            OpenCurlyBrace => CloseCurlyBrace,
            CloseCurlyBrace => OpenCurlyBrace,
            CloseAngleBracket => OpenAngleBracket,
            OpenAngleBracket => CloseAngleBracket,
        }
    }

    pub fn matches(&self, other: &Self) -> bool {
        self == &other.pair()
    }

    pub fn get_score(&self) -> u64 {
        use Tag::*;
        match self {
            OpenRoundParenthesis | OpenSquareBracket | OpenCurlyBrace | OpenAngleBracket => {
                panic!("Opening tag does not have score")
            }
            CloseParenthesis => 3,
            CloseSquareBracket => 57,
            CloseCurlyBrace => 1197,
            CloseAngleBracket => 25137,
        }
    }
}

impl TryFrom<char> for Tag {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Tag::*;
        match value {
            '(' => Ok(OpenRoundParenthesis),
            ')' => Ok(CloseParenthesis),
            '[' => Ok(OpenSquareBracket),
            ']' => Ok(CloseSquareBracket),
            '{' => Ok(OpenCurlyBrace),
            '}' => Ok(CloseCurlyBrace),
            '<' => Ok(OpenAngleBracket),
            '>' => Ok(CloseAngleBracket),
            _ => Err(()),
        }
    }
}
