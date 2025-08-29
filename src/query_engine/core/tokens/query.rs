use super::ParseResult;
use super::get_query::GetQuery;
use super::insert_column_query::InsertColumnQuery;
use super::insert_row_query::InsertRowQuery;
use super::set_query::SetQuery;

#[derive(Debug)]
pub struct Query {
    get_query: Option<GetQuery>,
    set_query: Option<SetQuery>,
    insert_column_query: Option<InsertColumnQuery>,
    insert_row_query: Option<InsertRowQuery>,
}

impl Query {
    pub fn parse(lexemes: &[String]) -> Option<Self> {
        match GetQuery::parse(lexemes) {
            ParseResult::Val(get_query) => {
                return Some(Query {
                    get_query: Some(get_query),
                    set_query: None,
                    insert_column_query: None,
                    insert_row_query: None,
                });
            }
            ParseResult::None => {}
            ParseResult::Err => return None,
        }
        match SetQuery::parse(lexemes) {
            ParseResult::Val(set_query) => {
                return Some(Query {
                    get_query: None,
                    set_query: Some(set_query),
                    insert_column_query: None,
                    insert_row_query: None,
                });
            }
            ParseResult::None => {}
            ParseResult::Err => return None,
        }
        match InsertColumnQuery::parse(lexemes) {
            ParseResult::Val(insert_column_query) => {
                return Some(Query {
                    get_query: None,
                    set_query: None,
                    insert_column_query: Some(insert_column_query),
                    insert_row_query: None,
                });
            }
            ParseResult::None => {}
            ParseResult::Err => return None,
        }
        match InsertRowQuery::parse(lexemes) {
            ParseResult::Val(insert_row_query) => {
                return Some(Query {
                    get_query: None,
                    set_query: None,
                    insert_column_query: None,
                    insert_row_query: Some(insert_row_query),
                });
            }
            ParseResult::None => {}
            ParseResult::Err => return None,
        }
        None
    }
    pub fn evaluate(&self, fields: &mut Vec<String>, rows: &mut Vec<Vec<String>>) {
        match &self.get_query {
            Some(get_query) => return get_query.evaluate(fields, rows),
            None => {}
        }
        match &self.set_query {
            Some(set_query) => return set_query.evaluate(fields, rows),
            None => {}
        }
        match &self.insert_column_query {
            Some(insert_column_query) => return insert_column_query.evaluate(fields, rows),
            None => {}
        }
        match &self.insert_row_query {
            Some(insert_row_query) => return insert_row_query.evaluate(fields, rows),
            None => {}
        }
    }
}
