use super::modification::Modification;

#[derive(Debug)]
pub struct Assignment {
  field_name: String,
  modification: Modification
}

impl Assignment {
  pub fn evaluate(&self, fields: &Vec<String>, row: &Vec<String>) {
    
  }
}
