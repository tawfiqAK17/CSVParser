use super::ParseResult;
use super::modification::Modification;
use super::value;
use crate::log_error;

#[derive(Debug)]
pub struct Assignment {
    field_name: String,
    modification: Modification,
}

impl Assignment {
    pub fn parse(lexemes: &[String], idx: usize) -> (ParseResult<Self>, usize) {
        match lexemes.get(idx) {
            Some(lexeme1) => {
                if let Some(field_name) = value::parse_field_name(lexeme1) {
                    match lexemes.get(idx + 1) {
                        Some(lexeme2) => match lexeme2.as_str() {
                            "=" => {
                                let (modification_parse_result, last_idx) =
                                    Modification::parse(lexemes, idx + 2);
                                match modification_parse_result {
                                    ParseResult::Val(modification) => {
                                        return (
                                            ParseResult::Val(Assignment {
                                                field_name,
                                                modification,
                                            }),
                                            last_idx,
                                        );
                                    }
                                    ParseResult::None => {
                                        log_error!("expecting a modification after the = key word");
                                        return (ParseResult::Err, idx);
                                    }
                                    ParseResult::Err => return (ParseResult::Err, idx),
                                }
                            }
                            _ => {
                                log_error!("expecting a = after the field name {field_name}");
                                return (ParseResult::Err, idx);
                            }
                        },
                        None => {
                            log_error!("expecting a = after the field name {field_name}");
                            return (ParseResult::Err, idx);
                        }
                    }
                } else {
                    return (ParseResult::None, idx);
                }
            }
            None => return (ParseResult::None, idx),
        }
    }
    pub fn evaluate(&self, fields: &Vec<String>, row: &mut Vec<String>) {
        match fields.iter().position(|f| *f == self.field_name) {
            Some(idx) => match self.modification.evaluate(fields, row) {
                Some(new_val) => row[idx] = new_val,
                None => {}
            },
            None => {
                log_error!("no field named {}", self.field_name);
            }
        }
    }
    pub fn insert_column_evaluation(
        &self,
        fields: &mut Vec<String>,
        rows: &mut Vec<Vec<String>>,
        where_clause_eval_results: &Vec<bool>,
    ) {
        match fields.iter().position(|f| *f == self.field_name) {
            Some(_) => {
                log_error!("the field name '{}' is already exist", self.field_name);
            }
            None => {
                fields.push(self.field_name.clone());
                for i in 0..rows.len() {
                    // if the where condition is true for the current line add the new field to it
                    // else it will be empty
                    if where_clause_eval_results[i] {
                        match self.modification.evaluate(fields, &rows[i]) {
                            Some(new_val) => rows[i].push(new_val),
                            None => rows[i].push("".to_string()),
                        }
                    } else {
                        rows[i].push("".to_string());
                    }
                }
            }
        }
    }
}
