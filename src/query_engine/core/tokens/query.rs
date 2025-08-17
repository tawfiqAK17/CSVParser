use super::ParseResult;
use super::get_query::GetQuery;
use super::set_query::SetQuery;

#[derive(Debug)]
pub struct Query {
    get_query: Option<GetQuery>,
    set_query: Option<SetQuery>,
}

impl Query {
    pub fn parse(lexemes: &[&String]) -> Option<Self> {
        match GetQuery::parse(lexemes) {
            ParseResult::Val(get_query) => {
                return Some(Query {
                    get_query: Some(get_query),
                    set_query: None,
                });
            }
            ParseResult::None => eprintln!("the only query implemented until now is get"),
            ParseResult::Err => return None,
        }
        None
    }
    pub fn evaluate(&self, fields: &mut Vec<String>, rows: &mut Vec<Vec<String>>) -> () {
        match &self.get_query {
            Some(get_query) => return get_query.evaluate(fields, rows),
            None => {}
        }
        match &self.set_query {
            Some(set_query) => return set_query.evaluate(fields, rows),
            None => {}
        }
        () // if there is no command return ()
    }
}
