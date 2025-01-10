use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::all_consuming;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;

use crate::parser::boolean::boolean;
use crate::parser::boolean::Boolean;
use crate::parser::identifier::identifier;
use crate::parser::identifier::Identifier;
use crate::prelude::ParseResult;

#[derive(Debug)]
pub enum Solidity {
    Const,
    Var,
    Type(Identifier),
}

#[derive(Debug)]
pub struct Assign {
    pub solidity: Solidity,
    pub identifier: Identifier,
    pub value: Boolean,
}

fn _var(input: &str) -> ParseResult<&str, Solidity> {
    tag("var")(input).map(|(remaining, _)| (remaining, Solidity::Var))
}

fn _const(input: &str) -> ParseResult<&str, Solidity> {
    tag("const")(input).map(|(remaining, _)| (remaining, Solidity::Const))
}

fn _type(input: &str) -> ParseResult<&str, Solidity> {
    identifier(input).map(|(remaining, value)| (remaining, Solidity::Type(value)))
}

fn _solidity(input: &str) -> ParseResult<&str, Solidity> {
    alt((_const, _var, _type))(input)
}

fn _identifier(input: &str) -> ParseResult<&str, Identifier> {
    preceded(multispace1, identifier)(input)
}

fn _equals(input: &str) -> ParseResult<&str, &str> {
    delimited(multispace0, tag("="), multispace0)(input)
}

fn _value(input: &str) -> ParseResult<&str, Boolean> {
    terminated(boolean, terminated(multispace0, tag(";")))(input)
}

pub fn assign(input: &str) -> ParseResult<&str, Assign> {
    all_consuming(pair(
        terminated(pair(_solidity, _identifier), _equals),
        _value,
    ))(input)
    .map(|(remaining, ((solidity, identifier), value))| {
        (
            remaining,
            Assign {
                solidity,
                identifier,
                value,
            },
        )
    })
}
