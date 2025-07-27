#[derive(Debug)]
pub enum Tokens {
  Query(),
  Command(String),     
  Selector(),    
  AllSelector(),    
  FieldList(),   
  WhereClause(), 
  Where(),
  Condition(),   
  Expression(),  
  Comparison(),  
  FunctionCall(), 
  LogicalOp(String),   
  ComparisonOp(String), 
  FunctionName(String), 
  Value(),       
  Literal(String),     
  Number(isize),      
  Boolean(bool),     
  List(Vec<String>),        
  FieldName(String),   
}

impl Clone for Tokens {
    fn clone(&self) -> Self {
        match self {
            Self::Query() => Self::Query(),
            Self::Command(arg0) => Self::Command(arg0.clone()),
            Self::Selector() => Self::Selector(),
            Self::AllSelector() => Self::AllSelector(),
            Self::FieldList() => Self::FieldList(),
            Self::WhereClause() => Self::WhereClause(),
            Self::Where() => Self::Where(),
            Self::Condition() => Self::Condition(),
            Self::Expression() => Self::Expression(),
            Self::Comparison() => Self::Comparison(),
            Self::FunctionCall() => Self::FunctionCall(),
            Self::LogicalOp(arg0) => Self::LogicalOp(arg0.clone()),
            Self::ComparisonOp(arg0) => Self::ComparisonOp(arg0.clone()),
            Self::FunctionName(arg0) => Self::FunctionName(arg0.clone()),
            Self::Value() => Self::Value(),
            Self::Literal(arg0) => Self::Literal(arg0.clone()),
            Self::Number(arg0) => Self::Number(arg0.clone()),
            Self::Boolean(arg0) => Self::Boolean(arg0.clone()),
            Self::List(arg0) => Self::List(arg0.clone()),
            Self::FieldName(arg0) => Self::FieldName(arg0.clone()),
        }
    }
}
