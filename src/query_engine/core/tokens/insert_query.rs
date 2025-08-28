use super::ParseResult;
use crate::log_error;
use super::assign_list::AssignList;

#[derive(Debug)]
pub struct InsertQuery {
    assign_list: AssignList,
}

impl InsertQuery {
    pub fn parse(lexemes: &[String]) -> ParseResult<Self> {
        match lexemes.get(0) {
            Some(lexeme) => {
                if lexeme == "insert" {
                    match AssignList::parse(lexemes, 1) {
                        ParseResult::Val(assign_list) => {
                            return ParseResult::Val(InsertQuery { assign_list });
                        }
                        ParseResult::None => {
                          log_error!("missing an assign list after the 'insert' key word");
                          return ParseResult::Err;
                        },
                        ParseResult::Err => return ParseResult::Err,
                    }
                } else {
                    return ParseResult::None;
                }
            }
            None => return ParseResult::None,
        }
    }

    pub fn evaluate(&self, fields: &mut Vec<String>, rows: &mut Vec<Vec<String>>) -> () {
      self.assign_list.insert_evaluation(fields, rows);
    }
}
