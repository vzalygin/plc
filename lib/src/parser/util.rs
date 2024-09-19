use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::one_of,
    error::{ContextError, ParseError},
    IResult, Parser,
};

pub fn separator<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, (), E> {
    alt((space_char, comment)).parse(inp)
}

fn space_char<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, (), E> {
    one_of(" \n\t\r").map(|_| {}).parse(inp)
}

fn comment<'s, E: ParseError<&'s str> + ContextError<&'s str>>(
    inp: &'s str,
) -> IResult<&'s str, (), E> {
    tag("#")
        .and(take_while(|x| x != '\n' && x != '\0'))
        .map(|_| {})
        .parse(inp)
}
