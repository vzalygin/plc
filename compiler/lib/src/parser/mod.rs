mod terms;
mod util;

use anyhow::Result;
use nom::{
    combinator::{all_consuming},
    error::{ContextError, ParseError, VerboseError},
    Finish, IResult, Parser,
};
use terms::terms;

use crate::{
    common::{Ast, Term},
    err::CompilerError,
};

pub fn parse<'s>(source: &'s str) -> Result<Ast, CompilerError<'s>> {
    let (_, tokens) = match axiom::<VerboseError<&'s str>>(source).finish() {
        Ok(v) => v,
        Err(e) => return Err(CompilerError::parser_error(source, e)),
    };

    Ok(Ast::from_terms(tokens))
}

fn axiom<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Vec<Term>, E> {
    all_consuming(terms).parse(inp)
}

#[cfg(test)]
mod tests {
    use crate::common::{Ast, Term};

    use super::*;

    #[test]
    fn empty() {
        let source = "";
        let exp = Ast { terms: vec![] };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn positive_int() {
        let source = "42";
        let exp = Ast {
            terms: vec![Term::Int(42)],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn negative_int() {
        let source = "-42";
        let exp = Ast {
            terms: vec![Term::Int(-42)],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn add() {
        let source = "+";
        let exp = Ast {
            terms: vec![Term::Add],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn mul() {
        let source = "*";
        let exp = Ast {
            terms: vec![Term::Mul],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn sub() {
        let source = "-";
        let exp = Ast {
            terms: vec![Term::Sub],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn div() {
        let source = "/";
        let exp = Ast {
            terms: vec![Term::Div],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn print() {
        let source = ".";
        let exp = Ast {
            terms: vec![Term::Print],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn dup() {
        let source = "dup";
        let exp = Ast {
            terms: vec![Term::Dup],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn drop() {
        let source = "drop";
        let exp = Ast {
            terms: vec![Term::Drop],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn take() {
        let source = "take";
        let exp = Ast {
            terms: vec![Term::Take],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn newline_at_the_end() {
        let source = "1\n";
        let exp = Ast {
            terms: vec![Term::Int(1)],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn multiline_code() {
        let source = "1\n
            60";
        let exp = Ast {
            terms: vec![Term::Int(1), Term::Int(60)],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn only_comment() {
        let source = "# a comment";
        let exp = Ast { terms: vec![] };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn comment_after_op() {
        let source = "1 # a comment";
        let exp = Ast {
            terms: vec![Term::Int(1)],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn comment_before_op() {
        let source = "# a comment
            1";
        let exp = Ast {
            terms: vec![Term::Int(1)],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn comment_between_op() {
        let source = "2 # a comment
            1";
        let exp = Ast {
            terms: vec![Term::Int(2), Term::Int(1)],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn _2_plus_3_sum_and_print() {
        let source = "2 3 + .";
        let exp = Ast {
            terms: vec![Term::Int(2), Term::Int(3), Term::Add, Term::Print],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn empty_list() {
        let source = "[]";
        let exp = Ast {
            terms: vec![Term::List { terms: vec![] }],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn list_with_terms() {
        let source = "[1 .]";
        let exp = Ast {
            terms: vec![Term::List {
                terms: vec![Term::Int(1), Term::Print],
            }],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn list_with_terms_between_terms() {
        let source = "+ [1 .] -";
        let exp = Ast {
            terms: vec![
                Term::Add,
                Term::List {
                    terms: vec![Term::Int(1), Term::Print],
                },
                Term::Sub,
            ],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn apply() {
        let source = "!";
        let exp = Ast {
            terms: vec![Term::Apply],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn list_apply() {
        let source = "[5 .]!";
        let exp = Ast {
            terms: vec![
                Term::List {
                    terms: vec![Term::Int(5), Term::Print],
                },
                Term::Apply,
            ],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn and() {
        let source = "and";
        let exp = Ast {
            terms: vec![Term::And],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn or() {
        let source = "or";
        let exp = Ast {
            terms: vec![Term::Or],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn equals() {
        let source = "==";
        let exp = Ast {
            terms: vec![Term::Equals],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn not_equals() {
        let source = "!=";
        let exp = Ast {
            terms: vec![Term::NotEquals],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn less() {
        let source = "<";
        let exp = Ast {
            terms: vec![Term::Less],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn less_equals() {
        let source = "<=";
        let exp = Ast {
            terms: vec![Term::LessEquals],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn greater() {
        let source = ">";
        let exp = Ast {
            terms: vec![Term::Greater],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn greater_equals() {
        let source = ">=";
        let exp = Ast {
            terms: vec![Term::GreaterEquals],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn _if() {
        let source = "?";
        let exp = Ast {
            terms: vec![Term::If],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn not() {
        let source = "not";
        let exp = Ast {
            terms: vec![Term::Not],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn bool() {
        let source = "b";
        let exp = Ast {
            terms: vec![Term::Bool],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn bind() {
        let source = ":test123";
        let exp = Ast {
            terms: vec![Term::Bind {
                identifier: "test123".to_string(),
            }],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn bind_begins_with_keyword() {
        let source = ":andTest";
        let exp = Ast {
            terms: vec![Term::Bind {
                identifier: "andTest".to_string(),
            }],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn bind_wrong_identifier_forbidden_char() {
        let source = ":1test123";
        let act = parse(source);
        assert!(act.is_err());
    }

    #[test]
    fn bind_wrong_identifier_keyword() {
        let source = ":and";
        let act = parse(source);
        assert!(act.is_err());
    }

    #[test]
    fn bind_wrong_identifier_keyword_() {
        let source = ":and!";
        let act = parse(source);
        println!("{:?}", act);
        assert!(act.is_err());
    }

    #[test]
    fn call() {
        let source = "test123";
        let exp = Ast {
            terms: vec![Term::Put {
                identifier: "test123".to_string(),
            }],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }
}
