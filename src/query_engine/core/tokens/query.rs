use indexmap::IndexMap;

use super::get_query::GetQuery;
use super::set_query::SetQuery;

pub struct Query {
    get_query: Option<GetQuery>,
    set_query: Option<SetQuery>,
}

impl Query {
  pub fn parse(lexemes: &[&String]) -> Option<Self> {
    match GetQuery::parse(lexemes) {
        Some(get_query) => return Some(Query { get_query: Some(get_query), set_query: None }),
        None => eprintln!("the only query implemented until now is get"),
    }
    None
  }
    pub fn evaluate(&self, columns: &mut IndexMap<String, Vec<String>>) -> () {
        match &self.get_query {
            Some(get_query) => return get_query.evaluate(&columns),
            None => {}
        }
        match &self.set_query {
            Some(set_query) => return set_query.evaluate(columns),
            None => {}
        }
        () // if there is no command return ()
    }
}
