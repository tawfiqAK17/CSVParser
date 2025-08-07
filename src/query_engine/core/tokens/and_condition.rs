use super::not_condition::NotCondition;
pub struct AndCondition {
    not_condition: NotCondition,
    and_condition: Option<Box<AndCondition>>,
}

impl AndCondition {
    pub fn parse(lexemes: &[&String], idx: usize) -> (Option<Self>, usize) {
        let (not_condition_option, last_idx) = NotCondition::parse(lexemes, idx);
        match not_condition_option {
            Some(not_condition) => {
                // if there is an andCondition we check if there is a orCondition after it by
                // checkin if next lexeme is "or"
                if let Some(lexeme) = lexemes.get(last_idx) {
                    if *lexeme == "and" {
                        let (and_condition_option, last_idx) = Self::parse(lexemes, idx);
                        match and_condition_option {
                            Some(and_condition) => {
                                return (
                                    Some(AndCondition {
                                        not_condition,
                                        and_condition: Some(Box::new(and_condition)),
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
                    Some(AndCondition {
                        not_condition,
                        and_condition: None,
                    }),
                    last_idx,
                );
            }
            None => return (None, last_idx),
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
