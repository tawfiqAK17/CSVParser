#[derive(Debug)]
pub struct SetQuery {}

impl SetQuery {
    pub fn evaluate(&self, fields: &mut Vec<String>, rows: &mut Vec<Vec<String>>) -> () {
        ()
    }
}
