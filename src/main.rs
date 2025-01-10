mod parser;
mod prelude;

use crate::parser::assign::assign;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string(".playground/one.mocha").expect("Something went wrong reading the file");
    let p = assign(contents.trim());
    match p {
        Ok((_, result)) => println!("{:#?}", result),
        Err(e) => panic!("{}", e),
    }
}
