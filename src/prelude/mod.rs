use nom::error::ErrorKind;
use std::result::Result;

pub type ParseResult<T, U, E = (T, ErrorKind)> = Result<(T, U), nom::Err<E>>;
