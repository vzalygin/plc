use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace1, one_of},
    combinator::{eof, value},
    error::{ContextError, ParseError, VerboseError},
    multi::{many1, many_m_n, separated_list0},
    sequence::terminated,
    Finish, IResult, Parser,
};

use crate::{
    common::{Ast, Opcode},
    err::CompilerError,
};

pub fn parse<'s>(source: &'s str) -> Result<Ast, CompilerError<'s>> {
    let (_, tokens) = match opcode_list::<VerboseError<&'s str>>(source).finish() {
        Ok(v) => v,
        Err(e) => return Err(CompilerError::parser_error(source, e).into()),
    };

    Ok(Ast::from_tokens(tokens))
}

fn opcode_list<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Vec<Opcode>, E> {
    terminated(
        separated_list0(multispace1, alt((int, add, sub, mul, div, print))),
        eof,
    )
    .parse(inp)
}

fn add<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Opcode, E> {
    value(Opcode::Add, tag("+")).parse(inp)
}

fn sub<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Opcode, E> {
    value(Opcode::Sub, tag("-")).parse(inp)
}

fn mul<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Opcode, E> {
    value(Opcode::Mul, tag("*")).parse(inp)
}

fn div<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Opcode, E> {
    value(Opcode::Div, tag("/")).parse(inp)
}

fn print<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Opcode, E> {
    value(Opcode::Print, tag(".")).parse(inp)
}

fn int<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Opcode, E> {
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

            Opcode::Int(integer)
        })
        .parse(inp)
}

#[cfg(test)]
mod tests {
    use crate::common::{Ast, Opcode};

    use super::*;

    #[test]
    fn parse_positive_int() {
        let source = "42";
        let exp = Ast {
            codes: vec![Opcode::Int(42)],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn parse_negative_int() {
        let source = "-42";
        let exp = Ast {
            codes: vec![Opcode::Int(-42)],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn parse_add() {
        let source = "+";
        let exp = Ast {
            codes: vec![Opcode::Add],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn parse_mul() {
        let source = "*";
        let exp = Ast {
            codes: vec![Opcode::Mul],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn parse_sub() {
        let source = "-";
        let exp = Ast {
            codes: vec![Opcode::Sub],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn parse_div() {
        let source = "/";
        let exp = Ast {
            codes: vec![Opcode::Div],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn parse_print() {
        let source = ".";
        let exp = Ast {
            codes: vec![Opcode::Print],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    #[ignore]
    fn correct_opcode_seq_1() {
        let source = "-42-";
        let exp = Ast {
            codes: vec![Opcode::Int(-42), Opcode::Sub],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    #[ignore]
    fn correct_opcode_seq_2() {
        let source = "-+42";
        let exp = Ast {
            codes: vec![Opcode::Sub, Opcode::Int(42)],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    #[ignore]
    fn correct_opcode_seq_3() {
        let source = "42.0";
        let exp = Ast {
            codes: vec![Opcode::Int(42), Opcode::Print, Opcode::Int(0)],
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
            codes: vec![Opcode::Int(2), Opcode::Int(3), Opcode::Add, Opcode::Print],
        };
        let act = parse(source);
        assert!(act.is_ok());
        let act = act.unwrap();
        assert_eq!(exp, act);
    }
}
