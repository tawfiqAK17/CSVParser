use indexmap::IndexMap;
#[derive(Debug)]
pub struct SetQuery {}

impl SetQuery {
    pub fn evaluate(&self, columns: &mut IndexMap<String, Vec<String>>) -> () {
        ()
    }
}
