use super::position::Position;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum TokenKind {
    Identifier(String),

    // Keywords
    True,
    False,
    None,
    Not,
    And,
    Or,

    // Builtin types
    Character(char),
    String(String),
    Integer(i32),
    Float(f32),
    Boolean(bool),

    // Operators
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Power,
    Assignment,
    Equals,
    NotEquals,
    Greater,
    GreaterEq,
    LessThan,
    LessThanEq,

    // Brackets
    RightParen,
    LeftParen,
    RightCurly,
    LeftCurly,
    RightBracket,
    LeftBracket,

    // Arrows
    Arrow,
    DoubleArrow,

    // Other
    NewLine,
}

impl TokenKind {
    pub fn str_to_identifier(identifier: &str) -> Option<Self> {
        match identifier {
            "true" => Some(Self::Boolean(true)),
            "false" => Some(Self::Boolean(false)),
            "none" => Some(Self::None),
            "not" => Some(Self::Not),
            "and" => Some(Self::And),
            "or" => Some(Self::Or),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    start: Position,
    end: Option<Position>,
}

impl Token {
    pub fn single(kind: TokenKind, position: Position) -> Self {
        Self {
            kind,
            start: position,
            end: None,
        }
    }

    pub fn multi(kind: TokenKind, start: Position, end: Position) -> Self {
        Self {
            kind,
            start,
            end: Some(end),
        }
    }
}
