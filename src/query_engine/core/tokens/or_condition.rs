use std::iter::Product;

use super::and_condition::AndCondition;
use super::ParseResult;

#[derive(Debug)]
pub struct OrCondition {
    and_condition: AndCondition,
    or_condition: Option<Box<OrCondition>>,
}

impl OrCondition {
    pub fn parse(lexemes: &[&String], idx: usize) -> (ParseResult<Self>, usize) {
        let (and_condition_parse_result, last_idx) = AndCondition::parse(lexemes, idx);
        match and_condition_parse_result {
            ParseResult::Val(and_condition) => {
                // if there is an andCondition we check if there is a orCondition after it by
                // checkin if next lexeme is "or"
                if let Some(lexeme) = lexemes.get(last_idx + 1) {
                    if *lexeme == "or" {
                        let (or_condition_parse_resutl, last_idx) = Self::parse(lexemes, last_idx + 2);
                        match or_condition_parse_resutl {
                            ParseResult::Val(or_condition) => {
                                return (
                                    ParseResult::Val(OrCondition {
                                        and_condition,
                                        or_condition: Some(Box::new(or_condition)),
                                    }),
                                    last_idx,
                                );
                            }
                            ParseResult::None => {
                                return (ParseResult::None, last_idx);
                            }
                            ParseResult::Err => return (ParseResult::Err, last_idx),
                        }
                    }
                }
                // if the next lexeme isn't "or" that means there is only an AndCondition
                return (
                    ParseResult::Val(OrCondition {
                        and_condition,
                        or_condition: None,
                    }),
                    last_idx,
                );
            }
            ParseResult::None => return (ParseResult::None, last_idx),
            ParseResult::Err => return (ParseResult::Err, last_idx),
        }
    }
    pub fn evaluate(&self, fields: &Vec<String>, row: &Vec<String>) -> bool {
        match &self.or_condition {
            Some(or_condition) => {
                return or_condition.evaluate(fields, row)
                    || self.and_condition.evaluate(fields, row);
            }
            None => {
                return self.and_condition.evaluate(fields, row);
            }
        }
    }
}
