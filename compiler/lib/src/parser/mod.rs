use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::one_of,
    combinator::{eof, value},
    error::{ContextError, ParseError, VerboseError},
    multi::{many0, many1, many_m_n, separated_list0},
    sequence::{delimited, terminated},
    Finish, IResult, Parser,
};

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
                alt((int, add, sub, mul, div, print))
            ),
            many0(separator)
        ),
        eof,
    )
    .parse(inp)
}

fn separator<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, (), E> {
    alt((space_char, comment)).parse(inp)
}

fn space_char<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, (), E> {
    one_of(" \n\t\r")
        .map(|_| {  })
        .parse(inp)
}

fn comment<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, (), E> {
    tag("#")
        .and(take_while(|x| { x != '\n' && x != '\0' }))
        .map(|_| {  })
        .parse(inp)
}

fn add<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Add, tag("+")).parse(inp)
}

fn sub<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Sub, tag("-")).parse(inp)
}

fn mul<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Mul, tag("*")).parse(inp)
}

fn div<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Div, tag("/")).parse(inp)
}

fn print<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Print, tag(".")).parse(inp)
}

fn int<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    many_m_n(0, 1, one_of("-+"))
        .and(many1(one_of("1234567890")))
        .map(|(sign, digits)| {
            let sign = if !sign.is_empty() && sign[0] == '-' {
                -1
            } else {
                1
            };
            let uinteger = digits
                .into_iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            let integer = sign * uinteger;

            Term::Int(integer)
        })
        .parse(inp)
}

#[cfg(test)]
mod tests {
    use crate::common::{Ast, Term};

    use super::*;

    #[test]
    fn empty() {
        let source = "";
        let exp = Ast {
            terms: vec![],
        };
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
        let source = 
            "1\n
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
        let exp = Ast {
            terms: vec![],
        };
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
        let source = 
            "# a comment
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
        let source = 
            "2 # a comment
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
