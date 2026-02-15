use crate::error::{Error, Result};
use crate::token::{Token, Tokenizer};
use crate::value::JsonValue;

pub fn parse(input: &str) -> Result<JsonValue> {
    Parser::new(input)?.parse()
}

struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    lookahead: Token,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Result<Self> {
        let mut tokenizer = Tokenizer::new(input);
        let lookahead = tokenizer.next_token()?;
        Ok(Self { tokenizer, lookahead })
    }

    fn parse(mut self) -> Result<JsonValue> {
        let value = self.parse_value()?;
        if self.lookahead == Token::EOF {
            Ok(value)
        } else {
            Err(Error::TrailingCharacters {
                pos: self.tokenizer.position(),
            })
        }
    }

    fn advance(&mut self) -> Result<()> {
        self.lookahead = self.tokenizer.next_token()?;
        Ok(())
    }

    fn position(&self) -> usize {
        self.tokenizer.position()
    }

    fn expect_symbol(&mut self, expected: Token, expected_name: &'static str) -> Result<()> {
        if self.lookahead == expected {
            self.advance()
        } else {
            Err(Error::ExpectedToken {
                expected: expected_name,
                pos: self.position(),
            })
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue> {
        match self.lookahead.clone() {
            Token::Null => {
                self.advance()?;
                Ok(JsonValue::Null)
            }
            Token::True => {
                self.advance()?;
                Ok(JsonValue::Bool(true))
            }
            Token::False => {
                self.advance()?;
                Ok(JsonValue::Bool(false))
            }
            Token::Number(n) => {
                self.advance()?;
                Ok(JsonValue::Number(n))
            }
            Token::String(s) => {
                self.advance()?;
                Ok(JsonValue::String(s))
            }
            Token::LBracket => self.parse_array(),
            Token::LBrace => self.parse_object(),
            _ => Err(Error::ExpectedValue {
                pos: self.position(),
            }),
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue> {
        self.expect_symbol(Token::LBracket, "[")?;
        let mut values = Vec::new();

        if self.lookahead == Token::RBracket {
            self.advance()?;
            return Ok(JsonValue::Array(values));
        }

        loop {
            values.push(self.parse_value()?);
            if self.lookahead == Token::Comma {
                self.advance()?;
                continue;
            }
            if self.lookahead == Token::RBracket {
                self.advance()?;
                break;
            }
            return Err(Error::ExpectedToken {
                expected: ", or ]",
                pos: self.position(),
            });
        }

        Ok(JsonValue::Array(values))
    }

    fn parse_object(&mut self) -> Result<JsonValue> {
        self.expect_symbol(Token::LBrace, "{")?;
        let mut members = Vec::new();

        if self.lookahead == Token::RBrace {
            self.advance()?;
            return Ok(JsonValue::Object(members));
        }

        loop {
            let key = match self.lookahead.clone() {
                Token::String(s) => {
                    self.advance()?;
                    s
                }
                _ => {
                    return Err(Error::ExpectedToken {
                        expected: "string key",
                        pos: self.position(),
                    });
                }
            };

            self.expect_symbol(Token::Colon, ":")?;
            let value = self.parse_value()?;
            members.push((key, value));

            if self.lookahead == Token::Comma {
                self.advance()?;
                continue;
            }
            if self.lookahead == Token::RBrace {
                self.advance()?;
                break;
            }
            return Err(Error::ExpectedToken {
                expected: ", or }",
                pos: self.position(),
            });
        }

        Ok(JsonValue::Object(members))
    }
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::error::Error;
    use crate::value::JsonValue;

    #[test]
    fn parses_nested_json() {
        let value = parse(r#"{"ok":true,"arr":[1,2,null,"x"]}"#).unwrap();
        assert_eq!(
            value,
            JsonValue::Object(vec![
                ("ok".to_string(), JsonValue::Bool(true)),
                (
                    "arr".to_string(),
                    JsonValue::Array(vec![
                        JsonValue::Number(1.0),
                        JsonValue::Number(2.0),
                        JsonValue::Null,
                        JsonValue::String("x".to_string()),
                    ]),
                ),
            ])
        );
    }

    #[test]
    fn rejects_trailing_characters() {
        let err = parse("true false").unwrap_err();
        assert!(matches!(err, Error::TrailingCharacters { .. }));
    }
}
