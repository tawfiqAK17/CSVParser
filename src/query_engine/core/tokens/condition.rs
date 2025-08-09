use super::ParseResult;

use super::or_condition::OrCondition;
#[derive(Debug)]
pub struct Condition {
    or_condition: OrCondition,
}

impl Condition {
    pub fn parse(lexemes: &[&String], idx: usize) -> (ParseResult<Self>, usize) {
        let (or_condition_parse_result, last_idx) = OrCondition::parse(lexemes, idx);
        match or_condition_parse_result {
            ParseResult::Val(or_condition) => {
                return (ParseResult::Val(Condition { or_condition }), last_idx);
            }
            ParseResult::None => return (ParseResult::None, last_idx),
            ParseResult::Err => return (ParseResult::Err, last_idx),
        }
    }
    pub fn evaluate(&self, fields: &Vec<&String>, row: &Vec<&String>) -> bool {
        return self.or_condition.evaluate(fields, row);
    }
}
