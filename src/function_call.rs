use std::collections::HashMap;

use super::*;

#[derive(Debug, Hash, PartialEq, Clone)]
pub struct FunctionCall {
    function_name: String,
    parameters: Vec<Expression>,
}


impl FunctionCall {

    pub fn new() -> FunctionCall {
        FunctionCall {
            function_name: String::new(),
            parameters: Vec::new(),
        }
    }

    pub fn from(name: String, parameters: Vec<Expression>) -> FunctionCall {
        FunctionCall {
            function_name: name,
            parameters: parameters,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.function_name
    }

    pub fn get_parameters(&self) -> &Vec<Expression> {
        &self.parameters
    }

    pub fn add_parameters(&mut self, params : &Vec<Expression>) {
        for p in params {
            self.parameters.push(p.clone());
        }
    }


    pub fn replace_variables(&self, 
        variables: &HashMap<String, Expression>) -> FunctionCall {

        let mut expressions = Vec::new();

        let fname = if let Some(Expression::FuncCall(expr)) = variables.get(&self.function_name) {
            for p in expr.get_parameters() {
                expressions.push(p.replace_variables(variables));
            }
            expr.get_name().clone()
        } else {
            self.function_name.clone()
        };

        for exp in &self.parameters {
            expressions.push(exp.replace_variables(variables));
        }

        return FunctionCall {
            function_name: fname,
            parameters: expressions,
        };
    }

}


