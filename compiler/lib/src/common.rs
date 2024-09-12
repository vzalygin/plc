#[derive(Clone, PartialEq, Debug)]
pub enum Term {
    Int(i32),
    Add,
    Sub,
    Mul,
    Div,
    Print,
    Dup,
    Drop,
    Take,
    List { terms: Vec<Term> },
    Apply,
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
