#[derive(Clone, PartialEq, Debug)]
pub enum Term {
    Int(i32),

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,

    Print,
    Scan,

    Dup,
    Drop,
    Take,

    // Lists
    List { terms: Vec<Term> },
    Apply,

    // Logical
    If,
    Bool,
    Not,
    And,
    Or,
    Equals,
    NotEquals,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,

    // Bindings
    Bind { identifier: String },
    Put { identifier: String },
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
