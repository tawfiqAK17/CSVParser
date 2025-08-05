use std::{env, io::stdin, process::exit};
use indexmap::IndexMap;
mod csv_parser;
mod query_engine;
mod logger;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: JCParser file-type [options] file");
        exit(1);
    }
    match args[1].as_str() {
        "csv" => println!("loading the csv file..."),
        "json" => println!("loading the json file..."),
        _ => {
            eprintln!("unknown file type {}", args[1]);
            exit(2);
        }
    }
    let mut columns: IndexMap<String, Vec<String>> = IndexMap::new();
    match csv_parser::parse_file("/home/tawfiq/test.csv") {
        Ok(val) => {
            columns = val;
        }
        Err(error) => eprintln!("{error}"),
    }

    // main loop
    loop {
        let mut command = String::new();
        println!("command: ");
        stdin()
            .read_line(&mut command)
            .expect("failed to read the command");

        query_engine::query(command.trim_end().to_string(), &columns);
        println!();
    }
}
