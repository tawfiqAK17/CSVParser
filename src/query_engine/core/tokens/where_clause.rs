use super::condition::Condition;
pub struct WhereClause {
    condition: Condition,
}

impl WhereClause {
    pub fn evaluate(&self, fields: Vec<&String>, row: &Vec<&String>) -> bool {
        return self.condition.evaluate(fields, row);
    }
}
