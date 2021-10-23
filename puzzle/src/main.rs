mod parser;
mod types;

use parser::parse;
// use types::Matrix;

fn main() {
    let m = parse();
    println!("{}", m.row);
    println!("{:?}", m.data);
    m.selfcheck();
}
