use super::primary_condition::PrimaryCondition;
pub struct NotCondition {
    not: Option<()>,
    primary_condition: PrimaryCondition,
}

impl NotCondition {
    pub fn parse(lexemes: &[&String], mut idx: usize) -> (Option<Self>, usize) {
        let mut not: Option<()> = None;
        if let Some(lexeme) = lexemes.get(idx) {
            if *lexeme == "not" {
                not = Some(());
                idx += 1;
            }
        } else {
            // it is the end of the lexemes
            return (None, idx);
        }
        match lexemes.get(idx) {
            Some(lexeme) => {
                let (primary_condition_option, last_idx) = PrimaryCondition::parse(lexemes, idx);
                match primary_condition_option {
                    Some(primary_condition) => {
                        return (Some(NotCondition {
                            not,
                            primary_condition,
                        }), last_idx);
                    }
                    None => {
                        return (None, idx);
                    }
                }
            }
            None => {
                eprintln!("expecting a condition after the not key word");
                return (None, idx);
            }
        }
    }
    pub fn evaluate(&self, fields: &Vec<&String>, row: &Vec<&String>) -> bool {
        match &self.not {
            Some(_) => return !self.primary_condition.evaluate(fields, row),
            None => return self.primary_condition.evaluate(fields, row),
        }
    }
}
