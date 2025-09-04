mod core;

pub fn query(query: String, fields: &mut Vec<String>, rows: &mut Vec<Vec<String>>) {

    // splitting the query to a vector of lexemes
    let lexemes: Vec<String> = query.split(' ').map(|s| s.to_string()).collect();
    
    core::query(&lexemes[..], fields, rows);
}
