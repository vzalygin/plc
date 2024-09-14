#[derive(Clone, PartialEq, Debug)]
pub enum Term {
    Int(i32),

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,

    Print,

    Dup,
    Drop,
    Take,

    // Lists
    List { terms: Vec<Term> },
    Apply,

    // Logical
    And,
    Or,
    Equals,
    NotEquals,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Ast {
    pub terms: Vec<Term>,
}

impl Ast {
    pub fn from_terms(terms: Vec<Term>) -> Ast {
        Ast { terms }
    }
}
