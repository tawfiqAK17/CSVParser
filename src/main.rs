use std::{env, io::stdin, process::exit};
mod csv_parser;
mod logger;
mod query_engine;

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
    let (mut fields, mut rows): (Vec<String>, Vec<Vec<String>>);
    match csv_parser::parse_file("/home/tawfiq/test.csv") {
        Some((f, r)) => {
            fields = f;
            rows = r;
        }
        None => return,
    }

    // main loop
    loop {
        let mut command = String::new();
        println!("command: ");
        match stdin().read_line(&mut command) {
            Ok(_) => {
                query_engine::query(command.trim_end().to_string(), &mut fields, &mut rows);
                println!();
            }
            Err(_) => {
                eprintln!("failed to read your command, please try again");
            }
        }
    }
}
