use super::and_condition::AndCondition;

#[derive(Debug)]
pub struct OrCondition {
    and_condition: AndCondition,
    or_condition: Option<Box<OrCondition>>,
}

impl OrCondition {
    pub fn parse(lexemes: &[&String], idx: usize) -> (Option<Self>, usize) {
        let (and_condition_option, last_idx) = AndCondition::parse(lexemes, idx);
        match and_condition_option {
            Some(and_condition) => {
                // if there is an andCondition we check if there is a orCondition after it by
                // checkin if next lexeme is "or"
                if let Some(lexeme) = lexemes.get(last_idx) {
                    if *lexeme == "or" {
                        let (or_condition_option, last_idx) = Self::parse(lexemes, idx);
                        match or_condition_option {
                            Some(or_condition) => {
                                return (
                                    Some(OrCondition {
                                        and_condition,
                                        or_condition: Some(Box::new(or_condition)),
                                    }),
                                    last_idx,
                                );
                            }
                            None => {
                                return (None, last_idx + 1);
                            }
                        }
                    }
                }
                // if the next lexeme isn't "or" that means there is only an AndCondition
                return (
                    Some(OrCondition {
                        and_condition,
                        or_condition: None,
                    }),
                    last_idx,
                );
            }
            None => return (None, last_idx),
        }
    }
    pub fn evaluate(&self, fields: &Vec<&String>, row: &Vec<&String>) -> bool {
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
