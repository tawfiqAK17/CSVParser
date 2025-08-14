use std::{
    fs::File,
    io::{BufRead, BufReader, stdin},
};

pub fn parse_file(path: &str) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    let file_result = File::open(path);
    let file: File;
    match file_result {
        Ok(val) => file = val,
        Err(_) => {
            eprintln!("can not open the file {path}");
            return None;
        }
    }

    let reader = BufReader::new(file);

    let mut get_fields = true;

    // the names of the fields
    let mut fields: Vec<String> = Vec::new();

    let mut rows: Vec<Vec<String>> = Vec::new();
    for line in reader.lines() {
        let mut line_content = String::new();
        match line {
            Ok(content) => line_content.push_str(&content),
            Err(_) => {
                eprintln!("an error accord while loading the file {path}");
                return None;
            }
        }
        if line_content.is_empty() {
            continue;
        }
        // if the fields are not extracted yet
        if get_fields {
            get_fields = false;
            let fields_vals: Vec<&str> = line_content.split(',').collect();
            for val in fields_vals {
                let field = val.trim().to_string();
                if field.is_empty() {
                    eprintln!("the name of a field should not be empty");
                    return None;
                }
                fields.push(val.trim().to_string());
            }
            continue;
        }

        let line_vals: Vec<&str> = line_content.split(',').collect();
        let mut row: Vec<String> = Vec::new();
        for mut i in 0..line_vals.len() {
            let val = line_vals[i].trim().to_string();
            if val.is_empty() {
                let mut new_val = String::new();
                println!("the value of the field {} is empty at line", fields[i]);
                println!("  {line_content}");
                println!(
                    "do you want to insert it here (enter the value to inset or press Return to escape):"
                );
                match stdin().read_line(&mut new_val) {
                    Ok(_) => {
                        new_val = new_val.trim().to_string();
                        if new_val.is_empty() {
                            println!(
                                "this line would not be considered and will be removed when you save the file"
                            );
                            row.clear();
                            break;
                        }
                        row.push(new_val);
                    }
                    Err(_) => {
                        eprintln!("there was an error while reading the new value try again");
                        i -= 1;
                        continue;
                    }
                }
            } else {
                row.push(val);
            }
        }
        if !row.is_empty() {
            rows.push(row);
        }
    }
    Some((fields, rows))
}
