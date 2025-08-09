use super::not_condition::NotCondition;
use super::ParseResult;

#[derive(Debug)]
pub struct AndCondition {
    not_condition: NotCondition,
    and_condition: Option<Box<AndCondition>>,
}

impl AndCondition {
    pub fn parse(lexemes: &[&String], idx: usize) -> (ParseResult<Self>, usize) {
        let (not_condition_parse_result, last_idx) = NotCondition::parse(lexemes, idx);
        match not_condition_parse_result {
            ParseResult::Val(not_condition) => {
                // if there is an andCondition we check if there is a orCondition after it by
                // checkin if next lexeme is "or"
                if let Some(lexeme) = lexemes.get(last_idx + 1) {
                    if *lexeme == "and" {
                        let (and_condition_parse_result, last_idx) = Self::parse(lexemes, last_idx + 2);
                        match and_condition_parse_result {
                            ParseResult::Val(and_condition) => {
                                return (
                                    ParseResult::Val(AndCondition {
                                        not_condition,
                                        and_condition: Some(Box::new(and_condition)),
                                    }),
                                    last_idx,
                                );
                            }
                            ParseResult::None => {
                                return (ParseResult::None, last_idx + 1);
                            }
                            ParseResult::Err => {
                                return (ParseResult::Err, last_idx + 1);
                            }
                        }
                    }
                }
                // if the next lexeme isn't "or" that means there is only an AndCondition
                return (
                    ParseResult::Val(AndCondition {
                        not_condition,
                        and_condition: None,
                    }),
                    last_idx,
                );
            }
            ParseResult::None => return (ParseResult::None, last_idx),
            ParseResult::Err => return (ParseResult::Err, last_idx),
        }
    }
    pub fn evaluate(&self, fields: &Vec<&String>, row: &Vec<&String>) -> bool {
        match &self.and_condition {
            Some(and_condition) => {
                return self.not_condition.evaluate(fields, row)
                    && and_condition.evaluate(fields, row);
            }
            None => {
                return self.not_condition.evaluate(fields, row);
            }
        }
    }
}
