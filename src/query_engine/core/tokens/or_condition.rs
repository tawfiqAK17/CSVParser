use super::and_condition::AndCondition;
pub struct OrCondition {
    and_condition: AndCondition,
    or_condition: Option<Box<OrCondition>>,
}
