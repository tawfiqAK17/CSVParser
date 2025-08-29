use super::ParseResult;
use super::assign_list::AssignList;
use super::where_clause::WhereClause;
use crate::log_error;

#[derive(Debug)]
pub struct InsertColumnQuery {
    where_clause: Option<WhereClause>,
    assign_list: AssignList,
}

impl InsertColumnQuery {
    pub fn parse(lexemes: &[String]) -> ParseResult<Self> {
        if let Some(lexeme) = lexemes.get(0) {
            if *lexeme != "insert-column" {
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
                        let (where_clause_parse_result, _) = WhereClause::parse(lexemes, last_idx);
                        match where_clause_parse_result {
                            ParseResult::Val(where_clause) => {
                                return ParseResult::Val(InsertColumnQuery {
                                    where_clause: Some(where_clause),
                                    assign_list,
                                });
                            }
                            ParseResult::None => {
                                return ParseResult::Val(InsertColumnQuery {
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
        let mut where_clause_eval_results: Vec<bool>;
        if let Some(where_clause) = &self.where_clause {
            where_clause_eval_results = vec![false; rows.len()];
            // this for loop will evaluate the where condition for every line
            for i in 0..rows.len() {
                where_clause_eval_results[i] = where_clause.evaluate(fields, &rows[i]);
            }
            self.assign_list
                .insert_column_evaluation(fields, rows, &where_clause_eval_results);
        } else {
            where_clause_eval_results = vec![true; rows.len()];
            self.assign_list
                .insert_column_evaluation(fields, rows, &where_clause_eval_results);
        }
    }
}
