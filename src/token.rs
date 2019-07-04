#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    None,
    White,
    Black,
}

impl Default for Token {
    fn default() -> Token {
        Token::None
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::None => write!(fmt, "  "),
            Token::White => write!(fmt, "▓▓"),
            Token::Black => write!(fmt, "░░"),
        }
    }
}
