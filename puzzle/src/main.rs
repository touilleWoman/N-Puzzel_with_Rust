mod a_star;
mod parser;
mod types;

extern crate getopts;
use getopts::Options;
use std::env;

use a_star::a_star;
use parser::parse;
use types::Heuristic;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("h", "heuristic", "choose heuristic in [manhatten](default) [euclidean] [tiles_out_of_place]", "euclidean");

    let matches = match opts.parse(&args[1..]) {
        Ok(x) => {x},
        Err(f) => {panic!("{}", f.to_string())},
    };
    let heu = match matches.opt_str("h") {
        None => "Manhattan".to_string(), // Manhattan is default heuristic
        Some(x) => x,
    };
    let heuristic = Heuristic::from_str(heu.trim()).unwrap();
    println!{"Use heuristic {:?}", heuristic};

    let m = parse("../test2");
    println!("{}", m.row);
    println!("{:?}", m.data);
    a_star(m, heuristic);
}
