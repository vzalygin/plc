pub enum TokenType {
    Int(i32),
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Token {
    value: TokenType,
    begin: usize,
    end: usize,
}

pub struct Ast<'source> {
    tokens: Vec<Token>,
    source: &'source str
}