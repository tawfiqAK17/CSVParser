use crate::log_error;
use crate::log_info;
use crate::log_warning;
use crate::{OPTIONS, Options};

use super::query_engine;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write, stdin, stdout},
};
pub fn run(path: &str) {
    // getting the content of the file
    let (mut fields, mut rows): (Vec<String>, Vec<Vec<String>>);
    match parse_file(path) {
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
                    print!("do you want to save the changes? (y or n): ");
                    let _ = stdout().flush();
                    command.clear();
                    // reading the user choice
                    match stdin().read_line(&mut command) {
                        Ok(_) => match command.trim_end() {
                            // the user want to save changes
                            "y" => {
                                // will open and clear the file that is already exist
                                let file = File::create(path);
                                match file {
                                    Ok(mut f) => {
                                        // write the fields names
                                        if let Err(e) = writeln!(f, "{}", fields.join(",")) {
                                            log_error!("Failed to write to file: {}", e);
                                            continue;
                                        }
                                        // write the rest of the rows
                                        for line in rows.iter() {
                                            if let Err(e) = writeln!(f, "{}", line.join(",")) {
                                                log_error!("Failed to write to file: {}", e);
                                                continue;
                                            }
                                        }
                                        log_info!("the changes has been written to: {}", path);
                                        return;
                                    }
                                    Err(e) => {
                                        log_error!("Failed to create file: {}", e);
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
                            log_error!("failed to read your choice");
                        }
                    }
                }
                query_engine::query(command.trim_end().to_string(), &mut fields, &mut rows);
                println!();
            }
            Err(_) => {
                log_error!("failed to read your command, please try again");
            }
        }
    }
}
pub fn parse_file(path: &str) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    let file_result = File::open(path);
    let file: File;
    match file_result {
        Ok(val) => file = val,
        Err(_) => {
            log_error!("can not open the file {path}");
            return None;
        }
    }
    // determining the separator
    let separator: String;
    let options = OPTIONS.get().unwrap();
    match options.get(&Options::FieldsSeparator) {
        Some(sep) => separator = sep.clone(),
        None => unreachable!("the default fields separator is not set"),
    }
    let reader = BufReader::new(file);

    let mut get_fields = true;

    // the names of the fields
    let mut fields: Vec<String> = Vec::new();

    let mut rows: Vec<Vec<String>> = Vec::new();
    log_info!("loading the csv file...");
    for line in reader.lines() {
        let mut line_content = String::new();
        match line {
            Ok(content) => line_content.push_str(&content),
            Err(_) => {
                log_error!("an error accord while loading the file {path}");
                return None;
            }
        }
        if line_content.is_empty() {
            continue;
        }
        // if the fields are not extracted yet
        if get_fields {
            get_fields = false;
            let fields_vals: Vec<&str> = line_content.split(separator.as_str()).collect();
            for val in fields_vals {
                let field = val.trim().to_string();
                if field.is_empty() {
                    log_error!("the name of a field should not be empty");
                    return None;
                }
                fields.push(val.trim().to_string());
            }
            continue;
        }

        let line_vals: Vec<&str> = line_content.split(separator.as_str()).collect();
        if line_vals.len() != fields.len() {
            log_warning!(
                "the line {} contains {} value, but there is {} field name",
                line_content,
                line_vals.len(),
                fields.len()
            );
            log_warning!("the line will be ignored");
            continue;
        }
        let mut row: Vec<String> = Vec::new();
        for mut i in 0..line_vals.len() {
            let val = line_vals[i].trim().to_string();
            row.push(val);
        }
        // so the empty lines will be escaped
        if !row.is_empty() {
            rows.push(row);
        }
    }
    Some((fields, rows))
}
