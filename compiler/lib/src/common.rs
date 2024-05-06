#[derive(Clone, PartialEq, Debug)]
pub enum Opcode {
    Int(i32),
    Add,
    Sub,
    Mul,
    Div,
    Print
}

#[derive(Clone, PartialEq, Debug)]
pub struct Ast {
    pub codes: Vec<Opcode>,
}

impl Ast {
    pub fn from_tokens(codes: Vec<Opcode>) -> Ast {
        Ast { codes }
    }
}
