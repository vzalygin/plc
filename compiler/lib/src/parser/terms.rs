use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::value,
    error::{ContextError, ParseError},
    multi::{many0, many1, many_m_n},
    sequence::delimited,
    IResult, Parser,
};

use crate::Term;

use super::util::separator;

pub fn terms<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Vec<Term>, E> {
    delimited(
        many0(separator),
        many0(term.and(many0(separator))).map(|term_pairs| {
            term_pairs
                .into_iter()
                .map(|term_pair| term_pair.0)
                .collect()
        }),
        many0(separator),
    )
    .parse(inp)
}

fn term<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    alt((
        int,
        add,
        sub,
        mul,
        div,
        print,
        dup,
        drop,
        take,
        list,
        and,
        or,
        not_equals,
        equals,
        less_equals,
        less,
        greater_equals,
        greater,
        apply,
    ))
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

fn dup<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Dup, tag("dup")).parse(inp)
}

fn drop<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Drop, tag("drop")).parse(inp)
}

fn take<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Take, tag("take")).parse(inp)
}

fn list<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    delimited(tag("["), terms, tag("]"))
        .map(|terms| Term::List { terms })
        .parse(inp)
}

fn apply<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Apply, tag("!")).parse(inp)
}

fn and<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::And, tag("and")).parse(inp)
}

fn or<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Or, tag("or")).parse(inp)
}

fn equals<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Equals, tag("==")).parse(inp)
}

fn not_equals<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::NotEquals, tag("!=")).parse(inp)
}

fn less<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Less, tag("<")).parse(inp)
}

fn less_equals<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::LessEquals, tag("<=")).parse(inp)
}

fn greater<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::Greater, tag(">")).parse(inp)
}

fn greater_equals<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, Term, E> {
    value(Term::GreaterEquals, tag(">=")).parse(inp)
}
