mod parser;
mod types;

use parser::parse;
// use types::*;

fn main() {
    let m = parse("../test2");
    println!("{}", m.row);
    println!("{:?}", m.data);
}
