use super::comparison::Comparison;
use super::condition::Condition;
pub struct PrimaryCondition {
    comparison: Option<Comparison>,
    condition: Option<Box<Condition>>,
}

impl PrimaryCondition {
    pub fn parse(lexemes: &[&String], mut idx: usize) -> (Option<Self>, usize) {
        match lexemes.get(idx) {
            Some(lexeme) => {
                if *lexeme == "(" {
                    idx += 1;
                    let (condition_option, last_idx) = Condition::parse(lexemes, idx);
                    match condition_option {
                        Some(condition) => match lexemes.get(last_idx) {
                            Some(lexeme) => {
                                if *lexeme == ")" {
                                    return (
                                        Some(PrimaryCondition {
                                            comparison: None,
                                            condition: Some(Box::new(condition)),
                                        }),
                                        last_idx + 1,
                                    );
                                } else {
                                    eprintln!("missing a ')' after the condition");
                                    return (None, last_idx);
                                }
                            }
                            None => {
                                eprintln!("missing a ')' after the condition");
                                return (None, last_idx);
                            }
                        },
                        None => {
                            eprintln!("missing a condition after the '('");
                            return (None, last_idx);
                        }
                    }
                } else {
                    let (comparison_option, last_idx) = Comparison::parse(lexemes, idx);
                    match comparison_option {
                        Some(comparison) => {
                            return (
                                Some(PrimaryCondition {
                                    comparison: Some(comparison),
                                    condition: None,
                                }),
                                last_idx,
                            );
                        }
                        None => {
                            return (None, idx);
                        }
                    }
                }
            }
            None => return (None, idx),
        }
    }
    pub fn evaluate(&self, fields: &Vec<&String>, row: &Vec<&String>) -> bool {
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
