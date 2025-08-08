use super::condition::Condition;
#[derive(Debug)]
pub struct WhereClause {
    condition: Condition,
}

impl WhereClause {
    pub fn parse(lexemes: &[&String], mut idx: usize) -> (Option<Self>, usize) {
        match lexemes.get(idx) {
            Some(val) => {
                if (*val == "where") {
                    let (option_condition, last_idx) = Condition::parse(lexemes, idx + 1);
                    match option_condition {
                        Some(condition) => return (Some(WhereClause { condition }), last_idx),
                        None => {
                            eprintln!("expecting a condition after the where key word");
                            return (None, idx);
                        }
                    }
                } else {
                    return (None, idx);
                }
            }
            None => return (None, idx),
        }
    }
    pub fn evaluate(&self, fields: &Vec<&String>, row: &Vec<&String>) -> bool {
        return self.condition.evaluate(fields, row);
    }
}
