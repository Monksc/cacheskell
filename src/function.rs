use super::*;

#[derive(Debug, Clone)]
pub struct Case {
    pub bool_expr   : Expression,
    pub return_expr : Expression,
}

#[derive(Debug, Clone)]
pub struct Function {
    cases: Vec<Case>,
    variables: Vec<String>,
}

impl Function {

    pub fn new() -> Function {
        Function {
            cases: Vec::new(),
            variables: Vec::new(),
        }
    }
    
    pub fn from(cases: Vec<Case>, variables: Vec<String>) -> Function {
        Function {
            cases: cases,
            variables: variables,
        }
    }

    pub fn get_variables(&self) -> &Vec<String> {
        &self.variables
    }

    pub fn get_cases(&self) -> &Vec<Case> {
        &self.cases
    }

}

