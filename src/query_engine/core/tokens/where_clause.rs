use super::ParseResult;

use super::condition::Condition;
#[derive(Debug)]
pub struct WhereClause {
    condition: Condition,
}

impl WhereClause {
    pub fn parse(lexemes: &[String], mut idx: usize) -> (ParseResult<Self>, usize) {
        match lexemes.get(idx) {
            Some(val) => {
                if (*val == "where") {
                    let (option_condition, last_idx) = Condition::parse(lexemes, idx + 1);
                    match option_condition {
                        ParseResult::Val(condition) => {
                            return (ParseResult::Val(WhereClause { condition }), last_idx);
                        }
                        ParseResult::None => {
                            eprintln!("expecting a condition after the where key word");
                            return (ParseResult::Err, idx);
                        }
                        ParseResult::Err => return (ParseResult::Err, idx),
                    }
                } else {
                  // there is no where key work
                    return (ParseResult::None, idx - 1);
                }
            }
            None => return (ParseResult::None, idx - 1),
        }
    }
    pub fn evaluate(&self, fields: &Vec<String>, row: &Vec<String>) -> bool {
        return self.condition.evaluate(fields, row);
    }
}
