mod a_star;
mod parser;
mod types;

use a_star::a_star;
use parser::parse;

fn main() {
    let m = parse("../test2");
    println!("{}", m.row);
    println!("{:?}", m.data);
    a_star(m);
}
