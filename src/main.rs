use std::{
    env,
    fs::File,
    io::{Write, stdin},
    process::{CommandArgs, exit},
};
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
    // getting the content of the file
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
                if command.trim_end() == "quit" {
                    println!("do you want to save the changes? (y or n)");
                    command.clear();
                    // reading the user choice
                    match stdin().read_line(&mut command) {
                        Ok(_) => match command.trim_end() {
                            // the user want to save changes
                            "y" => {
                                // will open and clear the file that is already exist
                                let file = File::create("/home/tawfiq/test.csv");
                                match file {
                                    Ok(mut f) => {
                                      // write the fields names
                                        if let Err(e) = writeln!(f, "{}", fields.join(",")) {
                                            eprintln!("Failed to write to file: {}", e);
                                            continue;
                                        }
                                        // write the rest of the rows
                                        for line in rows.iter() {
                                            if let Err(e) = writeln!(f, "{}", line.join(",")) {
                                                eprintln!("Failed to write to file: {}", e);
                                                continue;
                                            }
                                        }
                                        return;
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to create file: {}", e);
                                        continue;
                                    }
                                }
                            }
                            // the user don't want to save changes
                            "n" => {
                                return;
                            }
                            _ => {
                                println!("{}", command);
                                continue;
                            }
                        },
                        Err(_) => {
                            eprintln!("failed to read your choice");
                        }
                    }
                }
                query_engine::query(command.trim_end().to_string(), &mut fields, &mut rows);
                println!();
            }
            Err(_) => {
                eprintln!("failed to read your command, please try again");
            }
        }
    }
}
