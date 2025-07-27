use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parse_file(path: &str) -> Result<(Vec<String>, HashMap<String, Vec<String>>), Box<dyn std::error::Error>> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut get_fields = true;

    // the names of the fields
    let mut fields: Vec<String> = Vec::new();

    // the map would have the header name as the key and the column values as value;
    let mut columns: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines() {
        let mut line_content = String::new();
        match line {
            Ok(content) => line_content.push_str(&content),
            Err(error) => return Err(Box::new(error)),
        }
        // if the fields are not extracted yet
        if get_fields {
            get_fields = false;
            let fields_vals: Vec<&str> = line_content.split(',').collect();
            for val in fields_vals {
                fields.push(val.to_string());
                columns.insert(val.to_string(), Vec::new());
            }
            continue;
        }

        let line_vals: Vec<&str> = line_content.split(',').collect();
        for i in 0..line_vals.len() {
            if let Some(column) = columns.get_mut(&fields[i]) {
                column.push(line_vals[i].to_string());
            };
        }
    }

    Ok((fields, columns)) }
