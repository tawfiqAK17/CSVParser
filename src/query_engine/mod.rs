pub mod core;

pub fn query(query: String, fields: &mut Vec<String>, rows: &mut Vec<Vec<String>>) -> Result<(), String> {
    let temp_lexemes: Vec<String> = query.split(' ').map(|s| s.to_string()).collect();
    
    let lexemes: Vec<&String> = temp_lexemes.iter().collect();
    core::query(&lexemes[..], fields, rows);
    Ok(())
}
