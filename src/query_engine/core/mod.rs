mod tokens;

pub fn query(lexemes: &[&String], fields: &mut Vec<String>, rows: &mut Vec<Vec<String>>) {
    let query = tokens::query::Query::parse(lexemes);
    match query {
        Some(q) => {
            println!("{q:?}");
            q.evaluate(fields, rows);
        }
        None => {}
    }
}
