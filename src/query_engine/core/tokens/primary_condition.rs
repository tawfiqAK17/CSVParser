use super::ParseResult;
use super::comparison::Comparison;
use crate::log_error;
use super::condition::Condition;

#[derive(Debug)]
pub struct PrimaryCondition {
    comparison: Option<Comparison>,
    condition: Option<Box<Condition>>,
}

impl PrimaryCondition {
    pub fn parse(lexemes: &[String], mut idx: usize) -> (ParseResult<Self>, usize) {
        match lexemes.get(idx) {
            Some(lexeme) => {
                if *lexeme == "(" {
                  // the case where the condition is between brackets
                    idx += 1;
                    let (condition_parse_result, last_idx) = Condition::parse(lexemes, idx);
                    match condition_parse_result {
                        ParseResult::Val(condition) => match lexemes.get(last_idx + 1) {
                            Some(lexeme) => {
                                if *lexeme == ")" {
                                    return (
                                        ParseResult::Val(PrimaryCondition {
                                            comparison: None,
                                            condition: Some(Box::new(condition)),
                                        }),
                                        last_idx + 1,
                                    );
                                } else {
                                    log_error!("missing a ')' after the condition");
                                    return (ParseResult::Err, last_idx);
                                }
                            }
                            None => {
                                log_error!("missing a ')' after the condition");
                                return (ParseResult::Err, last_idx);
                            }
                        },
                        ParseResult::None => {
                            log_error!("missing a condition after the '('");
                            return (ParseResult::Err, last_idx);
                        }
                        ParseResult::Err => {
                            return (ParseResult::Err, last_idx);
                        }
                    }
                } else {
                  // the case where there is no brackets around the condition
                    let (comparison_parse_result, last_idx) = Comparison::parse(lexemes, idx);
                    match comparison_parse_result {
                        ParseResult::Val(comparison) => {
                            return (
                                ParseResult::Val(PrimaryCondition {
                                    comparison: Some(comparison),
                                    condition: None,
                                }),
                                last_idx,
                            );
                        }
                        ParseResult::None => {
                            return (ParseResult::None, idx);
                        }
                        ParseResult::Err => {
                            return (ParseResult::Err, idx);
                        }
                    }
                }
            }
            None => return (ParseResult::None, idx),
        }
    }
    pub fn evaluate(&self, fields: &Vec<String>, row: &Vec<String>) -> bool {
        match &self.comparison {
            Some(comparison) => return comparison.evaluate(fields, row),
            None => {}
        }
        match &self.condition {
            Some(condition) => return condition.evaluate(fields, row),
            None => {}
        }
        false
    }
}
