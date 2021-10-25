mod a_star;
mod parser;
mod types;

use a_star::a_star;
use parser::parse;
// use types::*;

fn main() {
    let m = parse("../test1");
    println!("{}", m.row);
    println!("{:?}", m.data);
    println!("value 0 is at :{:?}", m.position(0));
    a_star(m);
}
