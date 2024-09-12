use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    combinator::value,
    error::{ContextError, ParseError},
    multi::{many1, many_m_n},
    IResult, Parser,
};

use crate::Term;

pub fn add<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Add, tag("+")).parse(inp)
}

pub fn sub<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Sub, tag("-")).parse(inp)
}

pub fn mul<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Mul, tag("*")).parse(inp)
}

pub fn div<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Div, tag("/")).parse(inp)
}

pub fn print<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Print, tag(".")).parse(inp)
}

pub fn int<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
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

pub fn dup<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Dup, tag("dup")).parse(inp)
}

pub fn drop<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Drop, tag("drop")).parse(inp)
}

pub fn take<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Take, tag("take")).parse(inp)
}
