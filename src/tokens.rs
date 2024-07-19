use std::fmt;


#[derive(Debug,Clone)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
    
}
impl Token {
    pub fn new(kind: TokenType, value: String) -> Self {
        Self { kind, value}
    }
}

#[derive(Debug,PartialEq,Clone)]
pub enum TokenType {
    Type(String),
    String(String),
    Id,
    Number(f32),
    Negatives(f32),

    Whitespace(String),

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Plus,
    Minus,
    Star,
    Slash,
    Exp,
    Dot,
    Comma,
    Semicolon,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    PlusEqual,
    Greater,
    MinusEqual,
    FancyA,



    To,
    In,
    Import,
    For,
    While,
    Var,
    

    Eof,

}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Type(val) => write!(f, "Type({})", val),
            TokenType::String(val) => write!(f, "String({})", val),
            TokenType::Id => write!(f, "Id"),
            TokenType::Number(val) => write!(f, "Number({})", val),
            TokenType::Negatives(val) => write!(f, "Number({})", val),
            TokenType::Whitespace(val) => write!(f, "Whitespace({})", val),
            TokenType::LeftParen => write!(f, "LeftParen"),
            TokenType::RightParen => write!(f, "RightParen"),
            TokenType::LeftBrace => write!(f, "LeftBrace"),
            TokenType::RightBrace => write!(f, "RightBrace"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Star => write!(f, "Star"),
            TokenType::Slash => write!(f, "Slash"),
            TokenType::Exp => write!(f, " ^ "),
            TokenType::Dot => write!(f, "Dot"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::FancyA => write!(f, "FancyA"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Bang => write!(f, "Bang"),
            TokenType::BangEqual => write!(f, "BangEqual"),
            TokenType::Equal => write!(f, "Equal"),
            TokenType::EqualEqual => write!(f, "EqualEqual"),
            TokenType::Less => write!(f, "Less"),
            TokenType::MinusEqual => write!(f, "MinusEqual"),
            TokenType::Greater => write!(f, "Greater"),
            TokenType::PlusEqual => write!(f, "PlusEqual"),
            TokenType::To => write!(f, "To"),
            TokenType::In => write!(f, "In"),
            TokenType::Import => write!(f, "Import"),
            TokenType::For => write!(f, "For"),
            TokenType::While => write!(f, "While"),
            TokenType::Var => write!(f, "Var"),
            TokenType::Eof => write!(f, "Eof"),
        }
    }
}
