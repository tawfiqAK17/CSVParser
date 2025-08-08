mod tokens;
use indexmap::IndexMap;
pub fn query(lexemes: &[&String], columns: &mut IndexMap<String, Vec<String>>) {
    let query = tokens::query::Query::parse(lexemes);
    match query {
        Some(q) => {
println!("{:?}", q);
q.evaluate(columns);
        },
        None => {},
    }
}
