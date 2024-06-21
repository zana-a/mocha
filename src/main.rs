mod parser;
mod prelude;

use crate::parser::assign::assign;

fn main() {
    let p = assign("bool example = true;");
    match p {
        Ok((_, result)) => println!("{:#?}", result),
        Err(e) => panic!("{}", e),
    }
}
