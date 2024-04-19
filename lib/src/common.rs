#[derive(Clone)]
pub enum Token {
    Int(i32),
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Ast {
    pub tokens: Vec<Token>,
}
