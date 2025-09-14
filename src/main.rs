use regex::Regex;
use std::collections::HashMap;
use std::sync::OnceLock;
use std::{env, process::exit};
mod csv_parser;
mod logger;
mod query_engine;

pub static OPTIONS: OnceLock<HashMap<Options, String>> = OnceLock::new();

#[derive(Hash, PartialEq, Eq)]
pub enum Options {
    FieldsSeparator,
}

fn main() {
    // the regexs for all the possible options
    let fields_sep_regex = Regex::new(r"-s.").unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        log_error!("usage: CSVParser [options] file");
        exit(1);
    }

    let mut options: HashMap<Options, String> = HashMap::new();
    set_default_options(&mut options);
    // parse options
    for option in &args[2..args.len() - 1] {
        match option {
            op if fields_sep_regex.is_match(option) => {
                options.insert(Options::FieldsSeparator, op[2..].to_string())
            }
            _ => {
                log_error!("invalid option {}", option);
                return;
            }
        };
    }
    let _ = OPTIONS.set(options);
    csv_parser::run(args[args.len() - 1].as_str());
}

fn set_default_options(options: &mut HashMap<Options, String>) {
    options.insert(Options::FieldsSeparator, ",".to_string());
}
