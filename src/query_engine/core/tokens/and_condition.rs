use super::not_condition::NotCondition;
pub struct AndCondition {
  not_condition: NotCondition,
  and_condition: Option<Box<AndCondition>>,
}
