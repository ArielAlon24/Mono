use crate::models::error::{Error, InvalidSyntax};
use crate::models::position::Position;
use crate::models::token::{Token, TokenKind};

use core::iter::Peekable;

// crating a token with `None` end position
macro_rules! single {
    ($Position:expr, $TokenKind:expr) => {
        Some(Ok(Token::new($Position.clone(), None, $TokenKind)))
    };
}

// creating a token with `Some(Position)` end position.
macro_rules! multi {
    ($start:expr, $end:expr, $TokenKind:expr) => {
        Some(Ok(Token::new($start, Some($end.clone()), $TokenKind)))
    };
}

// creating a token with `None` or `Some(Position)` end position.
macro_rules! raw {
    ($start:expr, $end:expr, $TokenKind:expr) => {
        Some(Ok(Token::new($start, $end, $TokenKind)))
    };
}

macro_rules! error {
    ($ErrorKind:expr) => {
        Some(Err(Error::invalid_syntax($ErrorKind)))
    };
}

type IteratorItem = Option<Result<Token, Error>>;

pub struct Tokenizer<Chars: Iterator<Item = char>> {
    chars: Chars,
    current: IteratorItem,
    position: Position,
}

impl<Chars: Iterator<Item = char>> Tokenizer<Peekable<Chars>> {
    pub fn new(chars: Chars) -> Self {
        let mut tokenizer = Self {
            chars: chars.peekable(),
            current: None,
            position: Position::new(1, 0),
        };
        tokenizer.next();
        tokenizer
    }

    pub fn peek(&mut self) -> &IteratorItem {
        &self.current
    }

    pub fn get_position(&self) -> Position {
        self.position.clone()
    }

    fn _next(&mut self) -> IteratorItem {
        self.position.next();
        return match self.chars.next() {
            Some(' ') => self._next(),
            Some('\n') => {
                let token = single!(self.position, TokenKind::NewLine);
                self.position.newline();
                token
            }
            Some(c) if c.is_ascii_alphabetic() || c == '_' => self.next_identifier(c),
            Some(c) if c == '"' => self.next_string(),
            Some(c) if c == '\'' => self.next_char(),
            Some(c) if c.is_numeric() => self.next_number(c),
            Some('+') => single!(self.position, TokenKind::Addition),
            Some('-') => match self.chars.peek() {
                Some('>') => {
                    let start = self.get_position();
                    self.position.next();
                    self.chars.next();
                    multi!(start, self.position, TokenKind::Arrow)
                }
                _ => single!(self.position, TokenKind::Subtraction),
            },
            Some('*') => single!(self.position, TokenKind::Multiplication),
            Some('/') => single!(self.position, TokenKind::Division),
            Some('%') => single!(self.position, TokenKind::Modulo),
            Some('^') => single!(self.position, TokenKind::Power),
            Some('=') => match self.chars.peek() {
                Some('>') => {
                    let start = self.get_position();
                    self.chars.next();
                    self.position.next();
                    multi!(start, self.position, TokenKind::DoubleArrow)
                }
                Some('=') => {
                    let start = self.get_position();
                    self.chars.next();
                    self.position.next();
                    multi!(start, self.position, TokenKind::Equals)
                }
                _ => single!(self.position, TokenKind::Assignment),
            },
            Some('!') => match self.chars.next() {
                Some('=') => {
                    let start = self.get_position();
                    self.position.next();
                    multi!(start, self.position, TokenKind::NotEquals)
                }
                Some(_) => {
                    self.position.next();
                    error!(InvalidSyntax::UnexpectedChar(self.get_position(), '!'))
                }
                None => error!(InvalidSyntax::UnexpectedChar(self.get_position(), '!')),
            },
            Some('>') => match self.chars.peek() {
                Some('=') => {
                    let start = self.get_position();
                    self.chars.next();
                    self.position.next();
                    multi!(start, self.position, TokenKind::GreaterEq)
                }
                _ => single!(self.position, TokenKind::Greater),
            },
            Some('<') => match self.chars.peek() {
                Some('=') => {
                    let start = self.get_position();
                    self.chars.next();
                    self.position.next();
                    multi!(start, self.position, TokenKind::LessThanEq)
                }
                _ => single!(self.position, TokenKind::LessThan),
            },
            Some('(') => single!(self.position, TokenKind::LeftParen),
            Some(')') => single!(self.position, TokenKind::RightParen),
            Some('{') => single!(self.position, TokenKind::LeftCurly),
            Some('}') => single!(self.position, TokenKind::RightCurly),
            Some('[') => single!(self.position, TokenKind::LeftBracket),
            Some(']') => single!(self.position, TokenKind::RightBracket),
            Some(c) => error!(InvalidSyntax::UnrecognizedChar(self.get_position(), c)),
            _ => None,
        };
    }

    fn next_identifier(&mut self, c: char) -> IteratorItem {
        let start = self.get_position();
        let mut identifier = String::from(c);
        loop {
            match self.chars.peek() {
                Some(&c) if c.is_ascii_alphabetic() || c.is_numeric() || c == '_' => {
                    self.position.next();
                    self.chars.next();
                    identifier.push(c);
                }
                _ => break,
            }
        }

        let end = if &self.position == &start {
            None
        } else {
            Some(self.position.clone())
        };

        match TokenKind::str_to_identifier(&identifier) {
            Some(token_kind) => raw!(start, end, token_kind),
            _ => raw!(start, end, TokenKind::Identifier(identifier)),
        }
    }

    fn next_string(&mut self) -> IteratorItem {
        let start = self.get_position();
        let mut string = String::new();
        while let Some(&c) = self.chars.peek() {
            if c == '"' {
                break;
            }
            self.chars.next();
            self.position.next();
            string.push(c);
        }

        match self.chars.next() {
            Some(c) if c == '"' => multi!(start, self.position, TokenKind::String(string)),
            Some(_) => unreachable!(),
            None => {
                self.position.next();
                error!(InvalidSyntax::UnclosedCharDelimeter(
                    start,
                    self.get_position(),
                    '"',
                    None
                ))
            }
        }
    }

    fn next_char(&mut self) -> IteratorItem {
        let start = self.get_position();
        let result: char;

        match self.chars.next() {
            Some(c) => result = c,
            None => return error!(InvalidSyntax::UnexpectedChar(self.get_position(), '\'')),
        }

        self.position.next();
        return match self.chars.next() {
            Some('\'') => {
                self.position.next();
                multi!(start, self.position, TokenKind::Character(result))
            }
            Some(c) => {
                self.position.next();
                error!(InvalidSyntax::UnclosedCharDelimeter(
                    start,
                    self.get_position(),
                    '"',
                    Some(c),
                ))
            }
            None => {
                error!(InvalidSyntax::UnclosedCharDelimeter(
                    start,
                    self.get_position(),
                    '\'',
                    None
                ))
            }
        };
    }

    fn next_number(&mut self, c: char) -> IteratorItem {
        let start = self.get_position();
        let mut number = String::from(c);
        let mut is_float = false;

        loop {
            match self.chars.next() {
                Some(c) if c.is_numeric() => {
                    number.push(c);
                    self.position.next();
                }
                Some('.') => {
                    self.position.next();
                    if !is_float {
                        number.push('.');
                        is_float = true;
                    } else {
                        return error!(InvalidSyntax::MultipleFloatingPoints(
                            start,
                            self.get_position()
                        ));
                    }
                }
                _ => break,
            }
        }
        let end = if &self.position == &start {
            None
        } else {
            Some(self.position.clone())
        };

        if is_float {
            return match number.parse::<f32>() {
                Ok(float) => raw!(start, end, TokenKind::Float(float)),
                _ => panic!("Couldn't parse float: {:?}", number),
            };
        }
        match number.parse::<i32>() {
            Ok(int) => raw!(start, end, TokenKind::Integer(int)),
            _ => panic!("Couldn't parse integer: {:?}", number),
        }
    }
}

impl<Chars: Iterator<Item = char>> Iterator for Tokenizer<Peekable<Chars>> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Result<Token, Error>> {
        let current = self.current.take();
        self.current = self._next();
        current
    }
}
