use std::cmp::Ordering;
enum Value {
    Literal(String),
    FieldName(String),
    Number(f32),
    Boolean(bool),
    List(List),
}
enum ComparisonOps {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    BetweenOp,
    Is,
    IsNot,
    Contains,
    In,
    StartsWith,
    EndsWith,
}

enum BetweenOp {
    Min(Value),
    Max(Value),
}
struct List {}

pub struct Comparison {
    field_name: String,
    comparison_op: ComparisonOps,
    between_op: Option<BetweenOp>,
    rhs: Value,
}

impl Comparison {
    pub fn evaluate(&self, fields: Vec<&String>, row: &Vec<&String>) -> bool {
        match &self.comparison_op {
            ComparisonOps::Equal => match &&self.rhs {
                Value::FieldName(field) => match self.compaire_to_field_n(field, fields, row) {
                    Some(order) => match order {
                        Ordering::Equal => return true,
                        _ => return false,
                    },
                    None => return false,
                },
                Value::Number(val) => match self.compaire_to_Number(val.clone(), fields, row) {
                    Some(order) => match order {
                        Ordering::Equal => return true,
                        _ => return false,
                    },
                    None => return false,
                },
                _ => todo!(),
            },
            ComparisonOps::NotEqual => match &self.rhs {
                Value::FieldName(field) => match self.compaire_to_field_n(field, fields, row) {
                    Some(order) => match order {
                        Ordering::Equal => return false,
                        _ => return true,
                    },
                    None => return false,
                },
                Value::Number(val) => match self.compaire_to_Number(val.clone(), fields, row) {
                    Some(order) => match order {
                        Ordering::Equal => return false,
                        _ => return true,
                    },
                    None => return false,
                },
                _ => todo!(),
            },
            ComparisonOps::LessThan => match &self.rhs {
                Value::FieldName(field) => match self.compaire_to_field_n(field, fields, row) {
                    Some(order) => match order {
                        Ordering::Less => return true,
                        _ => return false,
                    },
                    None => return false,
                },
                Value::Number(val) => match self.compaire_to_Number(val.clone(), fields, row) {
                    Some(order) => match order {
                        Ordering::Less => return true,
                        _ => return false,
                    },
                    None => return false,
                },
                _ => todo!(),
            },

            ComparisonOps::GreaterThan => match &self.rhs {
                Value::FieldName(field) => match self.compaire_to_field_n(field, fields, row) {
                    Some(order) => match order {
                        Ordering::Greater => return true,
                        _ => return false,
                    },
                    None => return false,
                },
                Value::Number(val) => match self.compaire_to_Number(val.clone(), fields, row) {
                    Some(order) => match order {
                        Ordering::Greater => return true,
                        _ => return false,
                    },
                    None => return false,
                },
                _ => todo!(),
            },

            ComparisonOps::LessThanOrEqual => match &&self.rhs {
                Value::FieldName(field) => match self.compaire_to_field_n(field, fields, row) {
                    Some(order) => match order {
                        Ordering::Less | Ordering::Equal => return true,
                        _ => return false,
                    },
                    None => return false,
                },
                Value::Number(val) => match self.compaire_to_Number(val.clone(), fields, row) {
                    Some(order) => match order {
                        Ordering::Less | Ordering::Equal => return true,
                        _ => return false,
                    },
                    None => return false,
                },
                _ => todo!(),
            },
            ComparisonOps::GreaterThanOrEqual => match &self.rhs {
                Value::FieldName(field) => match self.compaire_to_field_n(field, fields, row) {
                    Some(order) => match order {
                        Ordering::Greater | Ordering::Equal => return true,
                        _ => return false,
                    },
                    None => return false,
                },
                Value::Number(val) => match self.compaire_to_Number(val.clone(), fields, row) {
                    Some(order) => match order {
                        Ordering::Greater | Ordering::Equal => return true,
                        _ => return false,
                    },
                    None => return false,
                },
                _ => todo!(),
            },
            ComparisonOps::BetweenOp => todo!(),
            ComparisonOps::Is => todo!(),
            ComparisonOps::IsNot => todo!(),
            ComparisonOps::Contains => todo!(),
            ComparisonOps::In => todo!(),
            ComparisonOps::StartsWith => todo!(),
            ComparisonOps::EndsWith => todo!(),
        }
    }
    // numerical comparison between two fields
    fn compaire_to_field_n(
        &self,
        field: &String,
        fields: Vec<&String>,
        row: &Vec<&String>,
    ) -> Option<Ordering> {
        let mut lhs_idx: usize;
        let mut rhs_idx: usize;
        match fields.iter().position(|&field| *field == self.field_name) {
            Some(idx) => lhs_idx = idx,
            None => {
                eprintln!("there is no field named {}", self.field_name);
                return None;
            }
        }
        match fields.iter().position(|&field| *field == *field) {
            Some(idx) => rhs_idx = idx,
            None => {
                eprintln!("there is no field named {}", self.field_name);
                return None;
            }
        }

        let mut lhs: f32;
        let mut rhs: f32;
        match row[lhs_idx].parse::<f32>() {
            Ok(val) => lhs = val,
            Err(_) => {
                eprintln!(
                    "{} is not a numerical value it has been evaluated as infinity",
                    row[lhs_idx]
                );
                return None;
            }
        }
        match row[rhs_idx].parse::<f32>() {
            Ok(val) => rhs = val,
            Err(_) => {
                eprintln!(
                    "{} is not a numerical value it has been evaluated as infinity",
                    row[rhs_idx]
                );
                return None;
            }
        }
        if lhs - rhs == 0f32 {
            return Some(Ordering::Equal);
        }
        if lhs - rhs > 0f32 {
            return Some(Ordering::Greater);
        }
        return Some(Ordering::Less);
    }
    fn compaire_to_Number(
        &self,
        number: f32,
        fields: Vec<&String>,
        row: &Vec<&String>,
    ) -> Option<Ordering> {
        let mut field_idx: usize;
        match fields.iter().position(|&field| *field == self.field_name) {
            Some(idx) => field_idx = idx,
            None => {
                eprintln!("there is no field named {}", self.field_name);
                return None;
            }
        }
        let mut field_val: f32;
        match row[field_idx].parse::<f32>() {
            Ok(val) => field_val = val,
            Err(_) => {
                eprintln!(
                    "{} is not a numerical value it has been evaluated as infinity",
                    row[field_idx]
                );
                return None;
            }
        }
        if field_val - number == 0f32 {
            return Some(Ordering::Equal);
        }
        if field_val - number > 0f32 {
            return Some(Ordering::Greater);
        }
        return Some(Ordering::Less);
    }
}
