use super::condition::Condition;
use super::comparison::Comparison;
pub struct PrimaryCondition {
  comparison: Option<Comparison>,
  condition: Option<Box<Condition>>,
}
