use super::or_condition::OrCondition;
#[derive(Debug)]
pub struct Condition {
    or_condition: OrCondition,
}

impl Condition {
    pub fn parse(lexemes: &[&String], idx: usize) -> (Option<Self>, usize) {
        let (or_condition_option, last_idx) = OrCondition::parse(lexemes, idx);
        match or_condition_option {
            Some(or_condition) => {
                return (Some(Condition { or_condition }), last_idx);
            }
            None => return (None, last_idx),
        }
    }
    pub fn evaluate(&self, fields: &Vec<&String>, row: &Vec<&String>) -> bool {
        return self.or_condition.evaluate(fields, row);
    }
}
