use super::ParseResult;
use super::delete_query::DeleteQuery;
use super::get_query::GetQuery;
use super::insert_column_query::InsertColumnQuery;
use super::insert_row_query::InsertRowQuery;
use super::set_query::SetQuery;
use super::aggregation_function_call::AggregationFunctionCall;

#[derive(Debug)]
pub struct Query {
    get_query: Option<GetQuery>,
    set_query: Option<SetQuery>,
    insert_column_query: Option<InsertColumnQuery>,
    insert_row_query: Option<InsertRowQuery>,
    delete_query: Option<DeleteQuery>,
    aggregation_function_call: Option<AggregationFunctionCall>,
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
                    delete_query: None,
                    aggregation_function_call: None,
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
                    delete_query: None,
                    aggregation_function_call: None,
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
                    delete_query: None,
                    aggregation_function_call: None,
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
                    delete_query: None,
                    aggregation_function_call: None,
                });
            }
            ParseResult::None => {}
            ParseResult::Err => return None,
        }
        match DeleteQuery::parse(lexemes) {
            ParseResult::Val(delete_query) => {
                return Some(Query {
                    get_query: None,
                    set_query: None,
                    insert_column_query: None,
                    insert_row_query: None,
                    delete_query: Some(delete_query),
                    aggregation_function_call: None,
                });
            }
            ParseResult::None => {}
            ParseResult::Err => return None,
        }
        match AggregationFunctionCall::parse(lexemes) {
            ParseResult::Val(aggregation_function_call) => {
                return Some(Query {
                    get_query: None,
                    set_query: None,
                    insert_column_query: None,
                    insert_row_query: None,
                    delete_query: None,
                    aggregation_function_call: Some(aggregation_function_call),
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
        match &self.delete_query {
            Some(delete_query) => return delete_query.evaluate(fields, rows),
            None => {}
        }
        match &self.aggregation_function_call {
            Some(aggregation_function_call) => return aggregation_function_call.evaluate(fields, rows),
            None => {},
        }
    }
}
