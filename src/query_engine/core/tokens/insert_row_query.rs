use super::ParseResult;
use super::assign_list::AssignList;
use crate::log_error;

#[derive(Debug)]
pub struct InsertRowQuery {
    assign_list: AssignList,
}

impl InsertRowQuery {
    pub fn parse(lexemes: &[String]) -> ParseResult<Self> {
        if let Some(lexeme) = lexemes.get(0) {
            if *lexeme != "insert-row" {
                return ParseResult::None;
            }
        } else {
            return ParseResult::None;
        }

        match lexemes.get(1) {
            Some(_) => {
                let (assign_list_parse_result, _) = AssignList::parse(lexemes, 1);
                match assign_list_parse_result {
                    ParseResult::Val(assign_list) => {
                        return ParseResult::Val(InsertRowQuery { assign_list });
                    }
                    ParseResult::None => {
                        log_error!("missing an assign list after the 'insert' key word");
                        return ParseResult::Err;
                    }
                    ParseResult::Err => return ParseResult::Err,
                }
            }
            None => return ParseResult::None,
        }
    }

    pub fn evaluate(&self, fields: &mut Vec<String>, rows: &mut Vec<Vec<String>>) -> () {
        self.assign_list.insert_row_evaluation(fields, rows);
    }
}
