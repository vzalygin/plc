use std::fmt::Display;

use nom::error::{convert_error, VerboseError};
use thiserror::Error;

#[derive(Clone, Error, Debug)]
pub enum CompilerError<'a> {
    ParserError {
        inp: &'a str,
        e: VerboseError<&'a str>,
    },
}

impl<'a> Display for CompilerError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerError::ParserError { inp: input, e } => {
                write!(f, "{}", convert_error(*input, e.to_owned()))
            }
        }
    }
}

impl<'a> CompilerError<'a> {
    pub fn parser_error(inp: &'a str, e: VerboseError<&'a str>) -> CompilerError<'a> {
        CompilerError::ParserError { inp, e }
    }
}