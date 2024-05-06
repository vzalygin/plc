use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while, take_while1},
    character::{complete::one_of, is_space},
    combinator::value,
    error::{ContextError, ParseError, VerboseError},
    multi::{many0, many1, many1_count, many_m_n},
    sequence::{preceded, terminated},
    IResult, Parser,
};

use crate::common::{Ast, Opcode};

pub fn parse<'s>(source: &'s str) -> Result<Ast> {
    let (_, tokens) = match token_vec::<VerboseError<&'s str>>(source) {
        Ok(res) => res,
        Err(e) => return Err(anyhow!(e.to_string())),
    };

    Ok(Ast::from_tokens(tokens))
}

fn token_vec<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Vec<Opcode>, E> {
    preceded(spaces, many0(token_and_space))(inp)
}

fn token_and_space<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Opcode, E> {
    terminated(alt((add, sub, mul, div, print, int)), spaces).parse(inp)
}

fn spaces<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, (), E> {
    take_while(|x: char| is_space(x as u8))
        .map(|_| ())
        .parse(inp)
}

fn add<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Opcode, E> {
    value(Opcode::Add, tag("add"))(inp)
}

fn sub<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Opcode, E> {
    value(Opcode::Sub, tag("sub"))(inp)
}

fn mul<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Opcode, E> {
    value(Opcode::Mul, tag("mul"))(inp)
}

fn div<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Opcode, E> {
    value(Opcode::Div, tag("div"))(inp)
}

fn print<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Opcode, E> {
    value(Opcode::Div, tag("."))(inp)
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
    use nom::error::VerboseError;

    use crate::{common::{Ast, Opcode}, parser::int};

    use super::parse;

    #[test]
    fn parse_positive_int() {
        let source = "42";
        let exp = Ast { codes: vec![Opcode::Int(42)] };
        let act = parse(source).unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn parse_negative_int() {
        let source = "-42";
        let exp = Ast { codes: vec![Opcode::Int(-42)] };
        let act = parse(source).unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn dont_parse_wrong_int_1() {
        let source = "-42-";
        assert!(parse(source).is_err());
    }

    #[test]
    fn dont_parse_wrong_int_2() {
        let source = "-+42";
        assert!(parse(source).is_err());
    }

    #[test]
    fn dont_parse_wrong_int_3() {
        let source = "42.0";
        assert!(parse(source).is_err());
    }

    #[test]
    fn _2_plus_3_sum_and_print() {
        let source = "+ .";
        let exp = Ast {
            codes: vec![Opcode::Int(2), Opcode::Int(3), Opcode::Add, Opcode::Print],
        };
        let act = parse(source).unwrap();
        assert_eq!(exp, act);
    }
}
