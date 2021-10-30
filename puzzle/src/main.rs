mod a_star;
mod parser;
mod types;

extern crate getopts;
use getopts::Options;
use std::env;

use a_star::a_star;
use parser::parse;
use types::Heuristic;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: ./{} [options]", program);
    print!{"{}", opts.usage(&brief)};
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt(
        "f",
        "file",
        "get starting board from a filepath",
        "Ex: ../tests/test3-1",
    );
    opts.optopt(
        "h",
        "heuristic",
        "choose heuristic in [manhatten](default) [euclidean] [tiles_out_of_place]",
        "Ex: euclidean",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(x) => x,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let heu = match matches.opt_str("h") {
        None => "Manhattan".to_string(), // Manhattan is default heuristic
        Some(x) => x,
    };
    let heuristic = Heuristic::from_str(heu.trim()).unwrap();
    println! {"Using heuristic {:?}", heuristic};

    let inputfile = matches.opt_str("f").unwrap();
    let m = parse(&inputfile);
    println!("Puzzuel size: {}\n{:?}", m.row, m.data);
    a_star(m, heuristic);
}
