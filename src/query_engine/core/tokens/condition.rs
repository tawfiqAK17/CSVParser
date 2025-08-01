use super::or_condition::OrCondition;
pub struct Condition {
  or_condition: OrCondition,
}

impl Condition {
   pub fn evaluate(&self,fields: Vec<&String>, row: &Vec<&String>) -> bool {
    true
   } 
}
