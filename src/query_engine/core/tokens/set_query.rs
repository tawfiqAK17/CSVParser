use indexmap::IndexMap;
pub struct SetQuery {}

impl SetQuery {
    pub fn evaluate(&self, columns: &mut IndexMap<String, Vec<String>>) -> () {
        ()
    }
}
