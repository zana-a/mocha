use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::satisfy;
use nom::character::{is_alphabetic, is_digit};
use nom::combinator::recognize;
use nom::multi::many0;
use nom::sequence::pair;

use crate::prelude::ParseResult;

#[derive(Debug)]
pub struct Identifier<'a> {
    pub value: &'a str,
}

fn alphabetic(input: &str) -> ParseResult<&str, &str> {
    recognize(satisfy(|c| is_alphabetic(c as u8)))(input)
}

fn numeric(input: &str) -> ParseResult<&str, &str> {
    recognize(satisfy(|c| is_digit(c as u8)))(input)
}

fn underscore(input: &str) -> ParseResult<&str, &str> {
    tag("_")(input)
}

fn head(input: &str) -> ParseResult<&str, &str> {
    alt((alphabetic, underscore))(input)
}

fn tail(input: &str) -> ParseResult<&str, &str> {
    recognize(many0(alt((alphabetic, underscore, numeric))))(input)
}

pub fn identifier(input: &str) -> ParseResult<&str, Identifier> {
    recognize(pair(head, tail))(input).map(|(input, value)| (input, Identifier { value }))
}
