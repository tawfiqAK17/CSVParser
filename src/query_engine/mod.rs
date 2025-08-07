pub mod core;
use indexmap::IndexMap;

pub fn query(query: String, columns: &IndexMap<String, Vec<String>>) -> Result<(), String> {
    let temp_lexemes: Vec<String> = query.split(' ').map(|s| s.to_string()).collect();
    
    let lexemes: Vec<&String> = temp_lexemes.iter().collect();
    core::query(&lexemes[..], columns);
    Ok(())
}
