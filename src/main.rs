use std::collections::HashMap;
use std::{env, process::exit};
mod csv_parser;
mod logger;
mod query_engine;

#[derive(Hash)]
enum Options {
    Separator,
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: JCParser -file_type [options] file");
        exit(1);
    }
    let mut options: HashMap<Options, Option<String>> = HashMap::new();
    match args[1].as_str() {
        "-csv" => {
            csv_parser::run(args[args.len() - 1].as_str());
        }
        "-json" => println!("loading the json file..."),
        _ => {
            eprintln!("unknown file type {}", args[1]);
            exit(2);
        }
    }
}
