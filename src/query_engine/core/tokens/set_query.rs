use super::ParseResult;
use super::assign_list::AssignList;
use super::where_clause::WhereClause;
use crate::log_error;

#[derive(Debug)]
pub struct SetQuery {
    where_clause: Option<WhereClause>,
    assign_list: AssignList,
}

impl SetQuery {
    pub fn parse(lexemes: &[String]) -> ParseResult<Self> {
        if let Some(lexeme) = lexemes.get(0) {
            if *lexeme != "set" {
                return ParseResult::None;
            }
        } else {
            return ParseResult::None;
        }
        match lexemes.get(1) {
            Some(_) => {
                let (assign_list_parse_result, last_idx) = AssignList::parse(lexemes, 1);
                match assign_list_parse_result {
                    ParseResult::Val(assign_list) => {
                        let (where_clause_parse_result, _) =
                            WhereClause::parse(lexemes, last_idx);
                        match where_clause_parse_result {
                            ParseResult::Val(where_clause) => {
                                return ParseResult::Val(SetQuery {
                                    where_clause: Some(where_clause),
                                    assign_list,
                                });
                            }
                            ParseResult::None => {
                                return ParseResult::Val(SetQuery {
                                    where_clause: None,
                                    assign_list,
                                });
                            }
                            ParseResult::Err => return ParseResult::Err,
                        }
                    }
                    ParseResult::None => {
                        log_error!("expecting an assign list after the set key word");
                        return ParseResult::Err;
                    }

                    ParseResult::Err => return ParseResult::Err,
                }
            }
            None => {
                log_error!("expecting an assign list after the set key word");
                return ParseResult::Err;
            }
        }
    }
    pub fn evaluate(&self, fields: &mut Vec<String>, rows: &mut Vec<Vec<String>>) -> () {
        if let Some(where_clause) = &self.where_clause {
            // this for loop will evaluate the where condition for every line
            for i in 0..rows.len() {
                if where_clause.evaluate(fields, &rows[i]) {
                    self.assign_list.set_evaluation(fields, &mut rows[i]);
                }
            }
        } else {
            for i in 0..rows.len() {
                self.assign_list.set_evaluation(fields, &mut rows[i]);
            }
        }
        ()
    }
}
