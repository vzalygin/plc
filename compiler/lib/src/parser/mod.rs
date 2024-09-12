mod terms;
mod util;

use anyhow::Result;
use nom::{
    branch::alt,
    combinator::eof,
    error::{ContextError, ParseError, VerboseError},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, terminated},
    Finish, IResult, Parser,
};
use terms::{add, div, drop, dup, int, mul, print, sub, take};
use util::separator;

use crate::{
    common::{Ast, Term},
    err::CompilerError,
};

pub fn parse<'s>(source: &'s str) -> Result<Ast, CompilerError<'s>> {
    let (_, tokens) = match term_list::<VerboseError<&'s str>>(source).finish() {
        Ok(v) => v,
        Err(e) => return Err(CompilerError::parser_error(source, e)),
    };

    Ok(Ast::from_terms(tokens))
}

fn term_list<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Vec<Term>, E> {
    terminated(
        delimited(
            many0(separator),
            separated_list0(
                many1(separator),
                alt((int, add, sub, mul, div, print, dup, drop, take)),
            ),
            many0(separator),
        ),
        eof,
    )
    .parse(inp)
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
}
