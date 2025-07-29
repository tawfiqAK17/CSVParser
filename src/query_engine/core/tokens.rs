#[derive(Debug)]
pub enum Tokens {
  Command(String),     
  AllSelector(),    
  Where(),
  LogicalOp(String),   
  ComparisonOp(String), 
  FunctionName(String), 
  Literal(String),     
  Number(isize),      
  Boolean(bool),     
  List(Vec<String>),        
  FieldName(String),   
}

impl Clone for Tokens {
    fn clone(&self) -> Self {
        match self {
            Self::Command(arg0) => Self::Command(arg0.clone()),
            Self::AllSelector() => Self::AllSelector(),
            Self::Where() => Self::Where(),
            Self::LogicalOp(arg0) => Self::LogicalOp(arg0.clone()),
            Self::ComparisonOp(arg0) => Self::ComparisonOp(arg0.clone()),
            Self::FunctionName(arg0) => Self::FunctionName(arg0.clone()),
            Self::Literal(arg0) => Self::Literal(arg0.clone()),
            Self::Number(arg0) => Self::Number(arg0.clone()),
            Self::Boolean(arg0) => Self::Boolean(arg0.clone()),
            Self::List(arg0) => Self::List(arg0.clone()),
            Self::FieldName(arg0) => Self::FieldName(arg0.clone()),
        }
    }
}
