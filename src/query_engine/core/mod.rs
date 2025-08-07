mod tokens;
use indexmap::IndexMap;
pub fn query(lexemes: &[&String], columns: &IndexMap<String, Vec<String>>) {
    tokens::query::Query::parse(lexemes);
}
