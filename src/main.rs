use getopts::Options;
use puzzle::*;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: ./{} [options]", program);
    print! {"{}", opts.usage(&brief)};
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("u", "unsolvable", "generate an unsolvable starting board");
    opts.optopt(
        "g",
        "generate",
        "generate a starting board radomly with given size",
        "Ex: 4",
    );
    opts.optopt(
        "i",
        "iteration",
        "define nb of iterations when generating the starting board, default is 1000",
        "Ex: 500",
    );
    opts.optopt(
        "f",
        "file",
        "get starting board from a filepath",
        "Ex: testfiless/test3-1",
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
            panic!("{}", f.to_string());
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let unsolvable = matches.opt_present("u");

    let opt_gen = matches.opt_str("g");
    let opt_file = matches.opt_str("f");
    let iteration = matches.opt_str("i");

    let (m, row) = if opt_gen.is_none() && opt_file.is_none() {
        println!("No starting board infos, generate default => puzzle of size 3");
        (generator::generator(3, 500, unsolvable), 3)
    } else if opt_gen.is_some() {
        let iter: i32 = match iteration {
            Some(x) => x.parse::<i32>().unwrap(),
            None => 1000,
        };
        let row = opt_gen.unwrap().parse::<i32>().unwrap();
        (generator::generator(row, iter, unsolvable), row)
    } else {
        parser::parse(&(opt_file.unwrap()))
    };
    if unsolvable_check::unsolvable_check(&m, row) {
        println!("solvable: false => \n{:?}", m.data);
        return;
    }

    let heu = match matches.opt_str("h") {
        None => "Manhattan".to_string(), // Manhattan is default heuristic
        Some(x) => x,
    };
    let heuristic = Heuristic::from_str(heu.trim()).unwrap();
    println! {"Using heuristic {:?}", heuristic};
    if a_star::a_star(m, heuristic, row) == None {
        println! {"solvable: false"};
    }
}
