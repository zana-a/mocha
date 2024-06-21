use nom::branch::alt;
use nom::bytes::complete::tag;

use crate::prelude::ParseResult;

#[derive(Debug)]
pub struct Boolean {
    pub value: bool,
}

fn _true(input: &str) -> ParseResult<&str, bool> {
    tag("true")(input).map(|(remaining, _)| (remaining, true))
}

fn _false(input: &str) -> ParseResult<&str, bool> {
    tag("false")(input).map(|(remaining, _)| (remaining, false))
}

pub fn boolean(input: &str) -> ParseResult<&str, Boolean> {
    alt((_true, _false))(input).map(|(remaining, value)| (remaining, Boolean { value }))
}
