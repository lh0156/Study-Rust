use crate::token::TokenError;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Tokenize(TokenError),
    ExpectedValue { pos: usize },
    ExpectedToken { expected: &'static str, pos: usize },
    TrailingCharacters { pos: usize },
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<TokenError> for Error {
    fn from(value: TokenError) -> Self {
        Self::Tokenize(value)
    }
}
