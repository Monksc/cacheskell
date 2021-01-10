use std::collections::HashMap;

use super::*;


#[derive(Debug, Hash, PartialEq, Clone)]
pub enum Expression {
    FuncCall (FunctionCall),
    Variable (String),
    Boolean (bool),
    Int (i64),
    Void,
}

impl Expression {

    pub fn replace_variables(&self, 
        variables: &HashMap<String, Expression>) -> Expression {

        if let Expression::Variable(v) = self {
            if let Some(e) = variables.get(v) {
                return e.clone();
            }
        } else if let Expression::FuncCall(c) = self {
            return Expression::FuncCall(c.replace_variables(&variables));
        } 

        return self.clone();
    }

}

