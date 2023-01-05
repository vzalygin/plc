pub enum Statement {
    PushValue(i32),
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    PopOut,
}

pub struct Ast {
    pub program: Vec<Statement>,
}
