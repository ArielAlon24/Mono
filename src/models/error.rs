use super::position::Position;
use crate::models::token::Token;
use crate::models::token::TokenKind;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidSyntaxError(InvalidSyntax),
}

#[derive(Debug, PartialEq)]
pub struct InvalidSyntax {
    kind: InvalidSyntaxKind,
    position: Position,
}

#[derive(Debug, PartialEq)]
pub enum InvalidSyntaxKind {
    UnclosedCharDelimeter(char, Option<char>),
    UnexpectedChar(char),
    MultipleFloatingPoints,
    UnrecognizedChar(char),
    InvalidToken(Vec<TokenKind>, Option<Token>),
}

impl InvalidSyntax {
    pub fn new(kind: InvalidSyntaxKind, position: Position) -> Self {
        Self { kind, position }
    }
}
