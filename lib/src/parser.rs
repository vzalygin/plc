use anyhow::Result;
use nom::{
    bytes::complete::{tag, take_till}, character::{complete::one_of, is_space}, combinator::value, error::{ContextError, ParseError}, multi::{many1, many1_count, many_m_n}, IResult, Parser
};

use crate::common::{Ast, Token};

pub fn parse(source: &str) -> Result<Ast> {
    todo!()
}

fn add<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str
) -> IResult<&'s str, Token, E> {
    value(Token::Add, tag("add")).parse(inp)
}

fn sub<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str
) -> IResult<&'s str, Token, E> {
    value(Token::Sub, tag("sub")).parse(inp)
}

fn mul<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str
) -> IResult<&'s str, Token, E> {
    value(Token::Mul, tag("mul")).parse(inp)
}

fn div<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str
) -> IResult<&'s str, Token, E> {
    value(Token::Div, tag("div")).parse(inp)
}

fn int<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str
) -> IResult<&'s str, Token, E> {
    many_m_n(0, 1, one_of("-+"))
        .and(many1(one_of("1234567890")))
        .map(|(sign, digits)| {
            let sign = if !sign.is_empty() && sign[0] == '-' {
                -1
            } else {
                1
            };
            let uinteger = digits
                .iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            let integer = sign * uinteger;

            Token::Int(integer)
        })
        .parse(inp)
}

fn space<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str
) -> IResult<&'s str, usize, E> {
    many1_count(take_till(|x: char| is_space(x as u8))).parse(inp)
}