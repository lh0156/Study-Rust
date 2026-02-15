//! JSON tokenizer.

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Punctuation
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]
    Colon,    // :
    Comma,    // ,

    // Literals
    Null,
    True,
    False,
    Number(f64),
    String(String),

    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenError {
    UnexpectedChar { ch: char, pos: usize },
    UnexpectedEOF,
    InvalidNumber { pos: usize },
    InvalidStringEscape { pos: usize },
    InvalidUnicodeEscape { pos: usize },
}

pub type TokResult<T> = Result<T, TokenError>;

pub struct Tokenizer<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input: input.as_bytes(), pos: 0 }
    }

    #[inline]
    fn peek(&self) -> Option<u8> { self.input.get(self.pos).copied() }

    #[inline]
    fn bump(&mut self) -> Option<u8> { let b = self.peek()?; self.pos += 1; Some(b) }

    #[inline]
    pub fn position(&self) -> usize { self.pos }

    pub fn skip_ws(&mut self) {
        while let Some(b) = self.peek() {
            match b {
                b' ' | b'\n' | b'\r' | b'\t' => self.pos += 1,
                _ => break,
            }
        }
    }

    fn consume_keyword(&mut self, keyword: &[u8], token: Token) -> TokResult<Token> {
        for &expected in keyword {
            match self.bump() {
                Some(actual) if actual == expected => {}
                Some(actual) => {
                    return Err(TokenError::UnexpectedChar {
                        ch: actual as char,
                        pos: self.pos.saturating_sub(1),
                    });
                }
                None => return Err(TokenError::UnexpectedEOF),
            }
        }
        if matches!(self.peek(), Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_')) {
            return Err(TokenError::UnexpectedChar {
                ch: self.peek().unwrap_or_default() as char,
                pos: self.pos,
            });
        }
        Ok(token)
    }

    fn read_hex4(&mut self) -> TokResult<u16> {
        let mut value = 0u16;
        for _ in 0..4 {
            let b = self.bump().ok_or(TokenError::UnexpectedEOF)?;
            let digit = match b {
                b'0'..=b'9' => b - b'0',
                b'a'..=b'f' => 10 + (b - b'a'),
                b'A'..=b'F' => 10 + (b - b'A'),
                _ => {
                    return Err(TokenError::InvalidUnicodeEscape {
                        pos: self.pos.saturating_sub(1),
                    });
                }
            };
            value = (value << 4) | u16::from(digit);
        }
        Ok(value)
    }

    fn read_unicode_escape(&mut self) -> TokResult<char> {
        let high = self.read_hex4()?;
        if (0xD800..=0xDBFF).contains(&high) {
            // High surrogate must be followed by \uXXXX low surrogate.
            if self.bump() != Some(b'\\') || self.bump() != Some(b'u') {
                return Err(TokenError::InvalidUnicodeEscape {
                    pos: self.pos.saturating_sub(1),
                });
            }
            let low = self.read_hex4()?;
            if !(0xDC00..=0xDFFF).contains(&low) {
                return Err(TokenError::InvalidUnicodeEscape {
                    pos: self.pos.saturating_sub(1),
                });
            }
            let high_ten = u32::from(high - 0xD800);
            let low_ten = u32::from(low - 0xDC00);
            let scalar = 0x10000 + ((high_ten << 10) | low_ten);
            return char::from_u32(scalar).ok_or(TokenError::InvalidUnicodeEscape {
                pos: self.pos.saturating_sub(1),
            });
        }
        if (0xDC00..=0xDFFF).contains(&high) {
            return Err(TokenError::InvalidUnicodeEscape {
                pos: self.pos.saturating_sub(1),
            });
        }
        char::from_u32(u32::from(high)).ok_or(TokenError::InvalidUnicodeEscape {
            pos: self.pos.saturating_sub(1),
        })
    }

    fn parse_string(&mut self) -> TokResult<Token> {
        self.bump(); // opening quote
        let mut out = String::new();
        let mut chunk_start = self.pos;

        loop {
            let b = self.peek().ok_or(TokenError::UnexpectedEOF)?;
            match b {
                b'"' => {
                    let chunk = std::str::from_utf8(&self.input[chunk_start..self.pos]).map_err(|_| {
                        TokenError::InvalidStringEscape { pos: self.pos }
                    })?;
                    out.push_str(chunk);
                    self.pos += 1;
                    return Ok(Token::String(out));
                }
                b'\\' => {
                    let chunk = std::str::from_utf8(&self.input[chunk_start..self.pos]).map_err(|_| {
                        TokenError::InvalidStringEscape { pos: self.pos }
                    })?;
                    out.push_str(chunk);
                    self.pos += 1; // consume '\'
                    let escape_pos = self.pos;
                    let escaped = self.bump().ok_or(TokenError::UnexpectedEOF)?;
                    match escaped {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{0008}'),
                        b'f' => out.push('\u{000C}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => out.push(self.read_unicode_escape()?),
                        _ => return Err(TokenError::InvalidStringEscape { pos: escape_pos }),
                    }
                    chunk_start = self.pos;
                }
                0x00..=0x1F => return Err(TokenError::InvalidStringEscape { pos: self.pos }),
                _ => self.pos += 1,
            }
        }
    }

    fn parse_number(&mut self) -> TokResult<Token> {
        let start = self.pos;

        if self.peek() == Some(b'-') {
            self.pos += 1;
        }

        match self.peek() {
            Some(b'0') => {
                self.pos += 1;
                if matches!(self.peek(), Some(b'0'..=b'9')) {
                    return Err(TokenError::InvalidNumber { pos: start });
                }
            }
            Some(b'1'..=b'9') => {
                self.pos += 1;
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }
            }
            _ => return Err(TokenError::InvalidNumber { pos: start }),
        }

        if self.peek() == Some(b'.') {
            self.pos += 1;
            let frac_start = self.pos;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
            if frac_start == self.pos {
                return Err(TokenError::InvalidNumber { pos: start });
            }
        }

        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.pos += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            let exp_start = self.pos;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
            if exp_start == self.pos {
                return Err(TokenError::InvalidNumber { pos: start });
            }
        }

        let text = std::str::from_utf8(&self.input[start..self.pos])
            .map_err(|_| TokenError::InvalidNumber { pos: start })?;
        let num = text
            .parse::<f64>()
            .map_err(|_| TokenError::InvalidNumber { pos: start })?;
        Ok(Token::Number(num))
    }

    pub fn next_token(&mut self) -> TokResult<Token> {
        self.skip_ws();
        let b = match self.peek() {
            Some(b) => b,
            None => return Ok(Token::EOF),
        };

        match b {
            b'{' => {
                self.pos += 1;
                Ok(Token::LBrace)
            }
            b'}' => {
                self.pos += 1;
                Ok(Token::RBrace)
            }
            b'[' => {
                self.pos += 1;
                Ok(Token::LBracket)
            }
            b']' => {
                self.pos += 1;
                Ok(Token::RBracket)
            }
            b':' => {
                self.pos += 1;
                Ok(Token::Colon)
            }
            b',' => {
                self.pos += 1;
                Ok(Token::Comma)
            }
            b'n' => self.consume_keyword(b"null", Token::Null),
            b't' => self.consume_keyword(b"true", Token::True),
            b'f' => self.consume_keyword(b"false", Token::False),
            b'"' => self.parse_string(),
            b'-' | b'0'..=b'9' => self.parse_number(),
            _ => Err(TokenError::UnexpectedChar {
                ch: b as char,
                pos: self.pos,
            }),
        }
    }
}

pub fn tokenize(input: &str) -> TokResult<Vec<Token>> {
    let mut tz = Tokenizer::new(input);
    let mut out = Vec::new();
    loop {
        let tok = tz.next_token()?;
        let is_eof = tok == Token::EOF;
        out.push(tok);
        if is_eof { break; }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::{tokenize, Token, TokenError};

    #[test]
    fn tokenizes_simple_object() {
        let tokens = tokenize(r#"{"a":1,"b":true}"#).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LBrace,
                Token::String("a".to_string()),
                Token::Colon,
                Token::Number(1.0),
                Token::Comma,
                Token::String("b".to_string()),
                Token::Colon,
                Token::True,
                Token::RBrace,
                Token::EOF,
            ]
        );
    }

    #[test]
    fn rejects_invalid_number() {
        let err = tokenize("01").unwrap_err();
        assert_eq!(err, TokenError::InvalidNumber { pos: 0 });
    }
}
